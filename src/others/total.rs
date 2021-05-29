use crate::others::FileDescriptor;
use std::fmt;

#[derive(Debug, Default, Clone, Copy)]
pub struct FileTotal {
    total_of_lines: u32,
    total_of_words: u32,
    pub total_of_chars: usize,
    pub total_of_bytes: usize,
}

impl FileTotal {
    pub fn calculate(&mut self, file: FileDescriptor) -> &mut Self {
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
