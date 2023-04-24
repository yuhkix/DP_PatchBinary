import os

def get_bytes(content):
    if type(content) == bytes:
        return content
    else:
        # convert string to bytes
        byte_string = bytes.fromhex(content)
        escaped_string = ''.join([f'\\x{byte:02x}' for byte in byte_string])
        byte_array = escaped_string.encode('utf-8').decode('unicode_escape').encode('latin-1')
        return bytes(byte_array)
    
def patch_at_offset(filepath, offset, patch):
    patch = get_bytes(patch) # set to bytes if not already
    with open(filepath, 'r+b') as fh:
        fh.seek(offset)
        fh.write(patch)

def patch_scan(filepath, find, replace):
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