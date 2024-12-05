# this script builds easyjs terminal cli.

import subprocess
import sys
import platform


platforms = {
    "windows": "windows",
    "win": "windows",
    "linux": "linux",
    "darwin": "mac",
    "macos": "mac",
    "ubuntu": "linux"
}


def get_platform():
    return platforms[platform.system().lower()]


args = sys.argv

# pos args
command = "help" # (help, build)
operating_system = get_platform() # (windows, linux, mac) (default is current system)

# flags
is_release = False # (--release, -r)

# get all args
for arg in args:
    if arg.startswith("-"):
        # check for flag.
        if arg == "-r" or arg == "--release":
            is_release = True
        else:
            print(f"Unknown flag: {arg}")
            exit(1)
    else:
        # check for pos args.
        if args.index(arg) == 1:
            command = arg.lower()
        elif args.index(arg) == 2:
            operating_system = arg.lower()
        else:
            continue


if command == "help":
    print(
"""
EasyJS build script.
Use this to build easyjs terminal cli from source.

Usage:
    python scripts/build.py [command] [operating_system]

Commands:
    help: This help message.
    build: Build easyjs terminal cli.
        operating_system: (windows, linux, mac) (default is current system)

Flags:
    -r, --release: Build easyjs terminal cli in release mode.
""")
    exit(0)
elif command == "build":
    subprocess.call([sys.executable, "scripts/load_std.py"])
    commands = ["cargo", "build"]
    if is_release:
        commands += ["--release"]
        # subprocess.call(["cargo", "build", "--release"])

    # get OS
    if operating_system == "windows":
        commands += ["--target", "x86_64-pc-windows-msvc"]
    elif operating_system == "linux":
        commands += ["--target", "x86_64-unknown-linux-gnu"]
    elif operating_system == "mac":
        commands += ["--target", "x86_64-apple-darwin"]
    else:
        print(f"Unknown operating system: {operating_system}")
        exit(1)

    subprocess.call(commands)
else:
    print(f"Unknown command: {command}")
    exit(1)