import os
import shutil

os.system("wasm-pack build --target web")

# check for bin
if not os.path.exists("bin"):
    os.mkdir("bin")

if not os.path.exists("bin/wasm"):
    os.mkdir("bin/wasm")

# this will create a pkg directory
shutil.copyfile("pkg/easyjs_bg.wasm", "bin/wasm/easyjs_bg.wasm")
shutil.copyfile("pkg/easyjs.js", "bin/wasm/easyjs.js")

shutil.rmtree("pkg")