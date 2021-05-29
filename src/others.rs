#![allow(dead_code, unused)]

mod options;
mod total;
pub mod util;

pub use self::options::FileOptions;
use self::total::FileTotal;

use std::{
    fmt::{self},
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Read, Write},
};

#[derive(Debug, Default, Clone)]
pub struct FileDescriptor {
    name: String,
    number_of_lines: u32,
    number_of_words: u32,
    number_of_chars: usize,
    byte_counts: usize,
}

impl FileDescriptor {
    fn file_stat(&mut self, file: File, filename: &str) -> Self {
        self.name = String::from(filename);
        {
            let mut file = File::open(filename.to_string()).unwrap();
            let mut buf = [0; 512];
            loop {
                let size = file.read(&mut buf).unwrap();
                if size == 0 {
                    break;
                }
                self.byte_counts += size;
            }
        }
        let buf = BufReader::new(file);
        for line in buf.lines() {
            self.number_of_lines += 1;
            for word in line.unwrap().split_whitespace() {
                self.number_of_words += 1;
                self.number_of_chars += word.chars().count();
            }
        }
        self.clone()
    }
}

impl fmt::Display for FileDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:^}: {:>5},  {:^}: {:>5},  {:^}: {:>5},  {:^}: {:>5}",
            "No. of Lines",
            self.number_of_lines,
            "No. of Words",
            self.number_of_words,
            "Byte Count",
            self.byte_counts,
            "Filename",
            self.name
        )
    }
}
