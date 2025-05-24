import os
import shutil

# Build the crate as WebAssembly
os.system("wasm-pack build --target web --release")

# Create output dirs
os.makedirs("bin/wasm", exist_ok=True)

# Copy WASM and JS glue code
shutil.copyfile("bin/wasm/easyjsc_bg.wasm", "bin/wasm/easyjsc_bg.wasm")
shutil.copyfile("bin/wasmpkg/easyjsc.js", "bin/wasm/easyjsc.js")
