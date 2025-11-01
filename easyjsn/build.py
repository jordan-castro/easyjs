import subprocess
import os
import re

# zig build-exe src/root.zig -target wasm32-freestanding -fno-entry --export=add

pattern = r'export\s+fn\s+(\w+)'
exports = []
with open('src/root.zig', 'r') as file:
    exports = re.findall(pattern, file.read())

cmd = "zig build-exe src/root.zig -target wasm32-freestanding -fno-entry"

for export in exports:
    cmd += ' '
    cmd += '--export=' + export

cmd += ' -OReleaseSmall'

print(cmd.split(' '))

subprocess.call(cmd.split(' '))
PATH = "../easyjsc/easyjsn.wasm"
if os.path.exists(PATH):
    os.remove(PATH)
os.rename('./root.wasm', PATH)