use std::{
    fs::File,
    io::{ErrorKind, Read},
};

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn print_or_create_file(path: &str) {
    let f = File::open(path);
    let mut f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create file: {:?}", e),
            }
        },
        Err(error) => {
            panic!("Failed to open file: {:?}", error)
        },
    };

    let mut buf = String::new();
    let _ = f.read_to_string(&mut buf);
    println!("{}", buf);
}
