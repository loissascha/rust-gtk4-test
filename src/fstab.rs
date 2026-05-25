use std::fs;
use std::io;

pub fn read_fstab() -> io::Result<String> {
    fs::read_to_string("/etc/fstab")
}
