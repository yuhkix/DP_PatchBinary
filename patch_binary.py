import os

def get_bytes(content):
    """
    Get bytes if input was for example a string.
    
    Args:
        content (string or bytes): can be a any of those
        
    Returns:
        bytes
    """
        
    if type(content) == bytes:
        return content
    else:
        # convert string to bytes
        byte_string = bytes.fromhex(content)
        escaped_string = ''.join([f'\\x{byte:02x}' for byte in byte_string])
        byte_array = escaped_string.encode('utf-8').decode('unicode_escape').encode('latin-1')
        return bytes(byte_array)
    
    
def patch_at_offset(filepath, offset, patch):
    """
    Patch bytes at specific offset.
    
    Args:
        filepath (string): the full path to the file you want to patch (.exe).
        offset (int or hexstring): the offset in your file
        patch (string or bytes): the new bytes you want to use.
        
    Returns:
        nothing.
    """

    patch = get_bytes(patch) # set to bytes if not already
    with open(filepath, 'r+b') as fh:
        fh.seek(offset)
        fh.write(patch)


def patch_scan(filepath, find, replace):
    # docstring
    """
    Patch bytes from 'find' with 'replace'.
    
    Args:
        filepath (string): the full path to the file you want to patch (.exe).
        find (string or bytes): the bytes you want to find and patch
        replace (string or bytes): the new bytes you want to use
        
    Returns:
        nothing.
    """

    find = get_bytes(find) # set to bytes if not already
    replace = get_bytes(replace) # set to bytes if not already
    if len(find) != len(replace):
        print("Find and replace must be same length")
    with open(filepath, 'r+b') as fh:
        content = fh.read()
        offset = content.find(find)
        if offset != -1:
            print("The pattern was found & patched at offset: ", hex(offset))
            patch_at_offset(filepath, offset, replace)
        else:
            print("The pattern can't be found.")