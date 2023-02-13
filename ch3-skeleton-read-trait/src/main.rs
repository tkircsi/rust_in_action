#![allow(unused_variables)]

use core::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl File {
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>) -> Self {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    pub fn open(self: &mut Self) -> Result<(), String> {
        self.state = FileState::Open;
        Ok(())
    }

    pub fn close(self: &mut Self) -> Result<(), String> {
        self.state = FileState::Closed;
        Ok(())
    }
}

impl Read for File {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let n_bytes = tmp.len();
        save_to.reserve(n_bytes);
        save_to.append(&mut tmp);
        Ok(n_bytes)
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

// fn open(f: File) -> Result<File, String> {
//     let new_file = File {
//         name: f.name,
//         data: f.data.clone(),
//         state: FileState::Open,
//     };
//     Ok(new_file)
// }

// fn close(f: File) -> Result<File, String> {
//     let new_file = File {
//         name: f.name,
//         data: f.data.clone(),
//         state: FileState::Closed,
//     };
//     Ok(new_file)
// }

fn main() {
    let mut f5 = File::new_with_data("5.txt", &vec![114, 117, 115, 116, 33]);
    let mut buffer = vec![];
    // f5 = open(f5).unwrap();
    f5.open().unwrap();
    let n_bytes = f5.read(&mut buffer).unwrap();
    f5.close().unwrap();
    // f5 = close(f5).unwrap();
    println!("{} byte(s) read from {}", n_bytes, f5);
}
