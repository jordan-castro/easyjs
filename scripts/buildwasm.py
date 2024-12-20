import os
import shutil

# os.system("wasm-pack build --target web")

# check for bin
if not os.path.exists("bin"):
    os.mkdir("bin")

if not os.path.exists("bin/wasm"):
    os.mkdir("bin/wasm")

# this will create a pkg directory
shutil.copyfile("pkg/easyjsc_bg.wasm", "bin/wasm/easyjsc_bg.wasm")
shutil.copyfile("pkg/easyjsc.js", "bin/wasm/easyjsc.js")

shutil.rmtree("pkg")