module EJImport

@enum FileType begin
    EJ = 1 # <-- Files ending with .ej
    JS = 2 # <-- Files ending with .js
    TS = 3 # <-- Files ending with .ts
    STD = 4 # <-- EasyJS standard library
    # JSON = 4
    # Python = 5 # <-- compiled to WASM.
end

"""
Get the file type.
"""
function file_type(path::String)
    ext = split(path, ".")[end]

    if ext == "ej"
        return EJ
    elseif ext == "js"
        return JS
    elseif ext == "ts"
        return TS
    else
        return STD
    end
end

end