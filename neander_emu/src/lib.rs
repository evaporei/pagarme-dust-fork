mod instruction_set;

use std::io::Bytes;
use std::io::prelude::*;
use std::fs::File;
use instruction_set::OpCode;

struct Parser {
    offset: u8,
    bytes: Bytes<File>,
}

#[derive(Debug, Clone)]
enum ParserError {
    InvalidOpCode(u8),
    MissingOperand(u8),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ParserError::InvalidOpCode(opcode) =>
                write!(f, "Invalid Operation Code: {}", opcode),
            ParserError::MissingOperand(opcode) =>
                write!(f, "Invalid Operand for Operation Code: {}", opcode),
        }
    }
}

impl std::error::Error for ParserError {
    fn description(&self) -> &str {
        "An error ocurred when parsing the binary"
    }
    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

impl Parser {
    fn new(file: File) -> Parser {
        let bytes = file.bytes();

        Parser {
            offset: 0,
            bytes,
        }
    }
    fn byte_to_opcode(&mut self) -> Result<OpCode, ParserError> {
        let byte = self.next().unwrap();
        let opcode = byte & 0xf0;

        match opcode {
            0x00 => Ok(OpCode::NOP),
            0x60 => Ok(OpCode::NOT),
            0xf0 => Ok(OpCode::HLT),
            _ => match self.next() {
                Some(operand) => match opcode {
                    0x10 => Ok(OpCode::STA(operand)),
                    0x20 => Ok(OpCode::LDA(operand)),
                    0x30 => Ok(OpCode::ADD(operand)),
                    0x40 => Ok(OpCode::OR(operand)),
                    0x50 => Ok(OpCode::AND(operand)),
                    0x80 => Ok(OpCode::JMP(operand)),
                    0x90 => Ok(OpCode::JN(operand)),
                    0xa0 => Ok(OpCode::JZ(operand)),
                    _ => Err(ParserError::InvalidOpCode(opcode)),
                },
                None => Err(ParserError::MissingOperand(opcode)),
            },
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
