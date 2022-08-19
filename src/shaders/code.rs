use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Code {
    pub string: Vec<u8>,
}

impl Code {
    pub fn new(path: &Path) -> Self {
        let mut buffer: Vec<u8> = vec![0; 1024 * 4];
        let written = File::open(path)
            .expect("Cant open the file.")
            .read(&mut buffer)
            .expect("Read file error.");

        /* null terminating cstring */
        let mut data: Vec<u8> = vec![0; written + 1];
        data.copy_from_slice(&buffer[0..=written]);
        Code { string: data }
    }
}
