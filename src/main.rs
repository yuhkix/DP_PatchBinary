use colored::*;
use std::fs::{self, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

// Parses a string of hexadecimal values into a vector of bytes.
// It can handle two formats:
// 1. Backslash-x notation (e.g., "\\x48\\x8D\\x05")
// 2. Space-separated hex values (e.g., "48 8D 05")
fn parse_hex_string(input: &str) -> Result<Vec<u8>, String> {
    let trimmed = input.trim();

    // Check for the "\\x" format first.
    if trimmed.contains("\\x") {
        let mut bytes: Vec<u8> = Vec::new();
        let mut chars = trimmed.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                if let Some('x') = chars.peek().copied() {
                    chars.next(); // Consume 'x'
                    // Take the next two characters for the hex code.
                    let h1 = chars.next().ok_or_else(|| {
                        "Invalid \\x sequence: missing first hex digit".to_string()
                    })?;
                    let h2 = chars.next().ok_or_else(|| {
                        "Invalid \\x sequence: missing second hex digit".to_string()
                    })?;
                    let hex = format!("{}{}", h1, h2);
                    let b = u8::from_str_radix(&hex, 16)
                        .map_err(|_| format!("Invalid hex value in \\x sequence: {}", hex))?;
                    bytes.push(b);
                }
            }
        }
        if !bytes.is_empty() {
            return Ok(bytes);
        }
    }

    // Fallback to parsing space-separated hex values.
    // This also handles Python-style byte string literals like b'...' or b"..."
    let mut core = trimmed.to_string();
    if core.starts_with("b'") && core.ends_with('\'') {
        core = core[2..core.len() - 1].to_string();
    } else if core.starts_with("b\"") && core.ends_with('"') {
        core = core[2..core.len() - 1].to_string();
    }

    let bytes: Result<Vec<u8>, _> = core
        .split_whitespace()
        .map(|part| u8::from_str_radix(part, 16))
        .collect();

    bytes.map_err(|e| format!("Invalid hex string: {}", e))
}

// Finds the starting position of a `needle` (byte slice) within a `haystack` (byte slice).
fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    // `windows` creates an iterator over overlapping sub-slices of the specified length.
    // `position` finds the index of the first item that matches the predicate.
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

// Writes a `patch` of bytes to a file at a specific `offset`.
fn patch_at_offset(filepath: &str, offset: u64, patch: &[u8]) -> Result<(), String> {
    // Open the file with read and write permissions.
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(filepath)
        .map_err(|e| format!("Failed to open file '{}': {}", filepath, e))?;

    // Move the file cursor to the specified offset.
    file.seek(SeekFrom::Start(offset))
        .map_err(|e| format!("Failed to seek to offset {:#x}: {}", offset, e))?;

    // Write the entire patch buffer.
    file.write_all(patch)
        .map_err(|e| format!("Failed to write patch: {}", e))?;

    Ok(())
}

// Scans a file for a sequence of bytes and replaces it with another.
fn patch_scan(filepath: &str, find: &str, replace: &str, patch_num: usize) {
    let header = format!("| Applying Patch #{}                  |", patch_num);
    println!("\n{}", "-------------------------------------".blue());
    println!("{}", header.blue());
    println!("{}", "-------------------------------------".blue());

    println!(
        "  {} Parsing 'find' and 'replace' byte patterns...",
        "->".yellow()
    );
    let find_bytes = match parse_hex_string(find) {
        Ok(b) => b,
        Err(e) => {
            eprintln!(
                "  {} {} {}",
                "✖".red(),
                "[ERROR]".red(),
                format!("Could not parse 'find' string: {}", e).red()
            );
            return;
        }
    };

    let replace_bytes = match parse_hex_string(replace) {
        Ok(b) => b,
        Err(e) => {
            eprintln!(
                "  {} {} {}",
                "✖".red(),
                "[ERROR]".red(),
                format!("Could not parse 'replace' string: {}", e).red()
            );
            return;
        }
    };

    if find_bytes.len() != replace_bytes.len() {
        eprintln!(
            "  {} {} Mismatch: 'Find' is {} bytes, but 'Replace' is {} bytes.",
            "✖".red(),
            "[ERROR]".red(),
            find_bytes.len(),
            replace_bytes.len()
        );
        eprintln!("            Patterns must be the same length. Aborting this patch.");
        return;
    }
    println!(
        "     {} Patterns are valid ({} bytes each).",
        "✔".green(),
        find_bytes.len()
    );

    print!(
        "  {} Reading file '{}' into memory... ",
        "->".yellow(),
        filepath
    );
    let mut content: Vec<u8> = Vec::new();
    match std::fs::File::open(filepath).and_then(|mut f| f.read_to_end(&mut content)) {
        Ok(bytes_read) => {
            println!("{} Read {} bytes.", "✔".green(), bytes_read);
        }
        Err(e) => {
            println!("{}", "✖ Failed!".red());
            eprintln!(
                "\n     {} {}",
                "[ERROR]".red(),
                format!("Could not read file '{}': {}", filepath, e).red()
            );
            return;
        }
    }

    print!("  {} Scanning for pattern... ", "->".yellow());
    if let Some(offset) = find_subslice(&content, &find_bytes) {
        println!("{} Found at offset {:#x}!", "✔".green(), offset);
        print!("  {} Applying patch... ", "->".yellow());
        match patch_at_offset(filepath, offset as u64, &replace_bytes) {
            Ok(_) => println!("{} Success!", "✔".green()),
            Err(e) => {
                println!("{}", "✖ Failed!".red());
                eprintln!(
                    "\n     {} {}",
                    "[ERROR]".red(),
                    format!("Could not apply patch: {}", e).red()
                );
            }
        }
    } else {
        println!("{}", "✖ Not Found.".yellow());
        println!(
            "     {} This pattern was not found. No changes made for this patch.",
            "[INFO]".cyan()
        );
    }
}

fn main() {
    println!(
        "\n{}",
        "=====================================".yellow().bold()
    );
    println!("{}", "=         BINARY PATCHER          =".yellow().bold());
    println!(
        "{}\n",
        "=====================================".yellow().bold()
    );

    let original = "dp_x64_original.exe";
    let patched = "dp_x64_patched.exe";

    print!(
        "{} Checking for original file '{}'... ",
        ">".bold(),
        original
    );
    if !Path::new(original).exists() {
        println!("{}", "✖ Not Found!".red());
        eprintln!(
            "\n{} {}",
            "[ERROR]".red(),
            format!("The file '{}' was not found.", original).red()
        );
        eprintln!("          Please place it in the same directory as this program.");
        println!("\nPress Enter to exit...");
        let _ = std::io::stdin().read_line(&mut String::new());
        std::process::exit(1);
    }
    println!("{}", "✔ Found!".green());

    print!("{} Creating a patchable copy '{}'... ", ">".bold(), patched);
    if let Err(e) = fs::copy(original, patched) {
        println!("{}", "✖ Failed!".red());
        eprintln!(
            "\n{} {}",
            "[ERROR]".red(),
            format!("Could not copy file: {}", e).red()
        );
        std::process::exit(1);
    }
    println!("{}", "✔ Done!".green());

    // Array of tuples, where each tuple contains a 'find' and 'replace' hex string.
    let disable_hwid_log: [(&str, &str); 2] = [
        (
            // Find pattern 1
            "4C 8D 05 82 FE 17 01 BA 0A 00 00 00 48 8D 4C 24 20 E8 F3 8F 4B 00 90 C7 44 24 30 E7 05 00 00 48 8D 4C 24 34 48 8B D3 41 B8 8C 01 00 00 E8 E3 B5 CD 00 4C 8D 44 24 30 BA 90 01 00 00 48 8B 0D 86 8F 8B 01 E8 F1 44 BC 00 90 48 8D 4C 24 20 E8 06 90 4B 00 48 8B 8C 24 C0 01 00 00 48 33 CC E8 96 B4 CD 00 48 81 C4 D0 01 00 00 5B",
            // Replace pattern 1 (with NOPs)
            "4C 8D 05 82 FE 17 01 BA 0A 00 00 00 48 8D 4C 24 20 90 90 90 90 90 90 C7 44 24 30 E7 05 00 00 48 8D 4C 24 34 48 8B D3 41 B8 8C 01 00 00 90 90 90 90 90 4C 8D 44 24 30 BA 90 01 00 00 48 8B 0D 86 8F 8B 01 90 90 90 90 90 90 48 8D 4C 24 20 90 90 90 90 90 48 8B 8C 24 C0 01 00 00 48 33 CC 90 90 90 90 90 48 81 C4 D0 01 00 00 5B",
        ),
        (
            // Find pattern 2
            "4C 8D 05 37 FE 17 01 BA 0A 00 00 00 48 8D 4C 24 58 E8 68 8F 4B 00 90 C7 44 24 28 ED 05 00 00 48 8D 4C 24 2C 8B 03 89 01 8B 43 04 89 41 04 8B 43 08 89 41 08 8B 43 0C 89 41 0C 8B 43 10 89 41 10 4C 8D 44 24 28 BA 18 00 00 00 48 8B 0D ED 8E 8B 01 E8 58 44 BC 00 90 48 8D 4C 24 58 E8 6D 8F 4B 00 48 83 C4 40 5B",
            // Replace pattern 2 (with NOPs)
            "4C 8D 05 37 FE 17 01 BA 0A 00 00 00 48 8D 4C 24 58 90 90 90 90 90 90 C7 44 24 28 ED 05 00 00 48 8D 4C 24 2C 8B 03 89 01 8B 43 04 89 41 04 8B 43 08 89 41 08 8B 43 0C 89 41 0C 8B 43 10 89 41 10 4C 8D 44 24 28 BA 18 00 00 00 48 8B 0D ED 8E 8B 01 90 90 90 90 90 90 48 8D 4C 24 58 90 90 90 90 90 48 83 C4 40 5B",
        ),
    ];

    // Loop through each patch and apply it.
    for (i, (find, replace)) in disable_hwid_log.iter().enumerate() {
        patch_scan(patched, find, replace, i + 1);
    }

    println!(
        "\n{}",
        "=====================================".green().bold()
    );
    println!("{}", "=      PATCHING COMPLETE          =".green().bold());
    println!("{}", "=====================================".green().bold());
    println!("\nThe file '{}' has been created/updated.", patched);
    println!("\nPress Enter to exit...");
    let _ = std::io::stdin().read_line(&mut String::new());
}
