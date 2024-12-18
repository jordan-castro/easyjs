# This script takes the .ej files from lib directory and places them in the rust std
# directory as strings. This will replace all current code in src/std/mod.rs

from glob import glob


def clean_file_contents(contents):
    contents = contents.replace("\t", " ")
    contents = contents.replace("\r", " ")

    return contents


version = ""
with open("lib/version", "r") as f:
    version = f.read()

lib_files = glob("lib/*.ej")

with open("src/std/mod.rs", "w") as f:
    f.write(f"// EasyJS STD version {version}\n")

    name_to_source = {}

    for file in lib_files:
        with open(file, "r") as lib_file:
            contents = clean_file_contents(lib_file.read())
            file = file.replace("\\", "/")
            name = file.split('/')[-1].split('.')[0]
            source = name.upper()

            name_to_source[name] = source

            f.write(
                f"const {file.split('/')[-1].split('.')[0].upper()}: &str = r##\"{contents}\"##;\n"
            )
    
    f.write("\n")
    f.write(f"/// Load a STD library from EasyJS version {version}, or an empty string if not found.\n")
    f.write("pub fn load_std(name: &str) -> String {\n")

    f.write("match name {\n")

    for name, source in name_to_source.items():
        f.write(f"\"{name}\" => {source},\n")

    f.write("_ => \"\",\n")
    f.write("}.to_string()")

    f.write("}")