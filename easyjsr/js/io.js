class File {
    static read(file_path) {
        return ___ejr_file_read(file_path);
    }

    static write(file_path, contents, encoding='utf-8') {
        ___ejr_file_write(file_path, contents, encoding);
    }
}

class Dir {
    static read(dir_path) {
        return ___ejr_dir_read(dir_path);
    }

    static is_dir(path) {
        return ___ejr_dir_is_dir(path);
    }
}
