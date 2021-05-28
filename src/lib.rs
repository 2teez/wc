#![allow(dead_code, unused)]

use std::{
    fmt::{self},
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Read, Write},
};

#[derive(Clone, Copy, std::cmp::PartialEq)]
pub enum FileOptions {
    Bytes,
    Chars,
    None,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct FileTotal {
    total_of_lines: u32,
    total_of_words: u32,
    total_of_chars: usize,
    total_of_bytes: usize,
}

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

impl FileTotal {
    fn calculate(&mut self, file: FileDescriptor) -> &mut Self {
        self.total_of_bytes += file.byte_counts;
        self.total_of_chars += file.number_of_chars;
        self.total_of_words += file.number_of_words;
        self.total_of_lines += file.number_of_lines;

        self
    }
}

impl fmt::Display for FileTotal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:^}: {:>5},  {:^}: {:>5},  {:^}: {:>5}   {:>5}",
            "No. of Lines",
            self.total_of_lines,
            "No. of Words",
            self.total_of_words,
            "Byte Count",
            self.total_of_bytes,
            "Total"
        )
    }
}

pub fn run(files: &[String], opt: FileOptions) {
    let mut file_total = FileTotal::default();

    if files.is_empty() {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("temp.txt");
        loop {
            let mut line = String::new();
            io::stdin().lock().read_line(&mut line);
            write!(file.as_ref().unwrap(), "{}", line);
        }
    } else {
        for file in files.iter() {
            let path = std::path::Path::new(&file);
            if path.is_file() {
                let file = File::open(file.to_string());
                let mut file_desc = FileDescriptor::default();

                let result_file_desc = file_desc
                    .file_stat(file.unwrap(), &path.file_name().unwrap().to_string_lossy());

                file_option_used(result_file_desc.clone(), opt);
                file_total.calculate(result_file_desc.clone());
            }
        }
        if files.len() >= 2 {
            println!("{}", "-".repeat(std::mem::size_of::<FileTotal>() * 4));
            match opt {
                FileOptions::Bytes => println!("\t{} Bytes Total", file_total.total_of_bytes),
                FileOptions::Chars => {
                    println!("\t{} Character(s) Total", file_total.total_of_chars)
                }
                FileOptions::None => println!("{}", file_total),
            }
        }
    }
}

pub fn file_option_used(st: FileDescriptor, opt: FileOptions) {
    match opt {
        FileOptions::Bytes => println!("\t{} Bytes {}", st.byte_counts, st.name),
        FileOptions::Chars => println!("\t{} Character(s) {}", st.number_of_chars, st.name),
        FileOptions::None => println!("{}", st),
    }
}
