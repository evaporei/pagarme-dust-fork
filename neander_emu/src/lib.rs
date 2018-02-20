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
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    fn test_next() {
        let file = File::open("binary_files/file1.dat").unwrap();
        let mut parser = Parser::new(file);

        assert_eq!(parser.next(), Some(0x61));
    }
}
