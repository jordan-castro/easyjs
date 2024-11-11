# this script builds easyjs termainal cli.

import subprocess
import sys


args = sys.argv
is_debug = True
if len(args) > 1 and args[1] == "release":
    is_debug = False

subprocess.call([sys.executable, "scripts/load_std.py"])
if is_debug:
    subprocess.call(["cargo", "build"])
else:
    subprocess.call(["cargo", "build", "--release"])
