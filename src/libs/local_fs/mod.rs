use std::{ffi::CString, fs};
use libc::statvfs64;
use std::path::Path;

pub fn folder_exists(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn file_exists(path: &str) -> bool {
    Path::new(path).is_file()
}

pub fn check_disk_space(path: &str) -> u64 {
    let c_path = CString::new(path).unwrap();
    let mut stat: statvfs64 = unsafe { std::mem::zeroed() };

    if unsafe { statvfs64(c_path.as_ptr(), &mut stat) } == 0 {
        let free_space = stat.f_bsize as u64 * stat.f_bavail as u64;
        free_space
    } else {
        0
    }
}

pub fn file_size(path: &str) -> u64 {
    fs::metadata(path).unwrap().len()
}

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

pub fn write_file(path: &str, content: &str) -> Result<(), std::io::Error> {
    fs::write(path, content)
}
