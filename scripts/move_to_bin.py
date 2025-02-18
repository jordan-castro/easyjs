# Add the release or debug to path

import os
import sys
import platform
import shutil


is_debug = True

if len(sys.argv) < 2:
    False


# look for the binary
ext = ".exe" if platform.system() == "Windows" else ""

path_to_binary = f"target/{"debug" if is_debug else "release"}/easyjs{ext}"

if not os.path.exists(path_to_binary):
    print(f"Could not find {path_to_binary}")


# save to bin/
if not os.path.exists("bin"):
    os.mkdir("bin")

shutil.copyfile(path_to_binary, f"bin/easyjs{ext}")