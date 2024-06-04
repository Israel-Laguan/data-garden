pub fn folder_exists(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn file_exists(path: &str) -> bool {
    Path::new(path).is_file()
}

pub fn check_disk_space(path: &str) -> u64 {
    // Implement disk space check
}

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

pub fn write_file(path: &str, content: &str) -> Result<(), std::io::Error> {
    fs::write(path, content)
}
