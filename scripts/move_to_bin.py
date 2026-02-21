# Add the release or debug to path

import os
import sys
import platform
import shutil


is_debug = False


# look for the binary
ext = ".exe" if platform.system() == "Windows" else ""

path_to_binary = f"target/{"debug" if is_debug else "release"}/easyjs{ext}"
print(path_to_binary)

if not os.path.exists(path_to_binary):
    print(f"Could not find {path_to_binary}")


shutil.copyfile(path_to_binary, f"easyjs{ext}")