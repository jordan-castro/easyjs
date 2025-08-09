import os
import shutil
import subprocess
from pathlib import Path

# === Configuration ===
version = "v0.4.5"
binary_name = "easyjs"  # Change this to match your `Cargo.toml` binary name

# List of targets to build
targets = {
    # "windows-x86": "i686-pc-windows-gnu",
    "windows-x64": "x86_64-pc-windows-gnu",
    "macos": "x86_64-apple-darwin",
    "linux": "x86_64-unknown-linux-gnu"
}

# === Directories ===
root_dir = Path(__file__).parent.parent.resolve()
release_dir = root_dir / "releases" / version
release_dir.mkdir(parents=True, exist_ok=True)

def run_command(cmd, env=None):
    print(f"Running: {' '.join(cmd)}")
    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env)
    if result.returncode != 0:
        print(result.stderr.decode())
        raise RuntimeError(f"Command failed: {' '.join(cmd)}")
    print(result.stdout.decode())

def build_for_target(target_name, target_triple):
    print(f"\n=== Building for {target_name} ({target_triple}) ===")

    # Build the project
    run_command(["cargo", "build", "--release", "--target", target_triple])

    # Determine file extension
    ext = ".exe" if "windows" in target_name else ""
    binary_path = root_dir / "target" / target_triple / "release" / (binary_name + ext)

    if not binary_path.exists():
        raise FileNotFoundError(f"Expected binary not found: {binary_path}")

    # Create target folder
    target_folder = release_dir / target_name
    target_folder.mkdir(parents=True, exist_ok=True)

    # Copy the binary
    output_binary = target_folder / (binary_name + ext)
    shutil.copy2(binary_path, output_binary)
    print(f"Copied to: {output_binary}")

def main():
    for target_name, target_triple in targets.items():
        build_for_target(target_name, target_triple)

    print(f"\n✅ All builds completed. Releases are in: {release_dir}")

if __name__ == "__main__":
    main()
