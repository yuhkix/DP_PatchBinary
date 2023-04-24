import os
import sys
import patch_binary as pb

# Change with your Path to dp_x64 
binary_file = r"C:\DPDev-Client\dp_x64.exe"


if not os.path.exists(binary_file):
    print('Error: File not found !')
    input('Press Enter Key to exit..')
    sys.exit()

"""
Pattern can be like:
'4C 8D 05 82'
or
b'\x4C\x8D\x05\x82'
""" 

# find : replace
# no more hwid (gpu serials) logging. 
# Server only get your IP and Mac Address
disable_hwid_log = {
    # SL_ClientComputerInfo
    '4C 8D 05 82 FE 17 01 BA 0A 00 00 00 48 8D 4C 24 20 E8 F3 8F 4B 00 90 C7 44 24 30 E7 05 00 00 48 8D 4C 24 34 48 8B D3 41 B8 8C 01 00 00 E8 E3 B5 CD 00 4C 8D 44 24 30 BA 90 01 00 00 48 8B 0D 86 8F 8B 01 E8 F1 44 BC 00 90 48 8D 4C 24 20 E8 06 90 4B 00 48 8B 8C 24 C0 01 00 00 48 33 CC E8 96 B4 CD 00 48 81 C4 D0 01 00 00 5B' : 
    '4C 8D 05 82 FE 17 01 BA 0A 00 00 00 48 8D 4C 24 20 90 90 90 90 90 90 C7 44 24 30 E7 05 00 00 48 8D 4C 24 34 48 8B D3 41 B8 8C 01 00 00 90 90 90 90 90 4C 8D 44 24 30 BA 90 01 00 00 48 8B 0D 86 8F 8B 01 90 90 90 90 90 90 48 8D 4C 24 20 90 90 90 90 90 48 8B 8C 24 C0 01 00 00 48 33 CC 90 90 90 90 90 48 81 C4 D0 01 00 00 5B',
    
    # SL_ClientPerformance
    '4C 8D 05 37 FE 17 01 BA 0A 00 00 00 48 8D 4C 24 58 E8 68 8F 4B 00 90 C7 44 24 28 ED 05 00 00 48 8D 4C 24 2C 8B 03 89 01 8B 43 04 89 41 04 8B 43 08 89 41 08 8B 43 0C 89 41 0C 8B 43 10 89 41 10 4C 8D 44 24 28 BA 18 00 00 00 48 8B 0D ED 8E 8B 01 E8 58 44 BC 00 90 48 8D 4C 24 58 E8 6D 8F 4B 00 48 83 C4 40 5B' : 
    '4C 8D 05 37 FE 17 01 BA 0A 00 00 00 48 8D 4C 24 58 90 90 90 90 90 90 C7 44 24 28 ED 05 00 00 48 8D 4C 24 2C 8B 03 89 01 8B 43 04 89 41 04 8B 43 08 89 41 08 8B 43 0C 89 41 0C 8B 43 10 89 41 10 4C 8D 44 24 28 BA 18 00 00 00 48 8B 0D ED 8E 8B 01 90 90 90 90 90 90 48 8D 4C 24 58 90 90 90 90 90 48 83 C4 40 5B',
}


# find : replace
for key in disable_hwid_log:
    pb.patch_scan(binary_file, key, disable_hwid_log[key])

input('Press Enter Key to exit..')