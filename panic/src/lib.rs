use std::fd::File;

pub fn open_file(s: &str) -> File {
    File::open(s).unwrap()
}
