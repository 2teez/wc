use crate::others::options::FileOptions;
use crate::others::total::FileTotal;
use crate::others::FileDescriptor;

use std::{
    fmt::{self},
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Read, Write},
};

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

fn file_option_used(st: FileDescriptor, opt: FileOptions) {
    match opt {
        FileOptions::Bytes => println!("\t{} Bytes {}", st.byte_counts, st.name),
        FileOptions::Chars => println!("\t{} Character(s) {}", st.number_of_chars, st.name),
        FileOptions::None => println!("{}", st),
    }
}
