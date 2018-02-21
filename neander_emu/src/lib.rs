use std::io::Bytes;
use std::io::prelude::*;
use std::fs::File;

struct Parser {
    offset: u8,
    bytes: Bytes<File>,
}

impl Parser {
    fn new(file: File) -> Parser {
        let bytes = file.bytes();

        Parser {
            offset: 0,
            bytes,
        }
    }
}

impl Iterator for Parser {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.offset += 1;

        match self.bytes.next() {
            Some(result) => Some(result.unwrap()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests;
