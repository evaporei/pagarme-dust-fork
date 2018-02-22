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
    fn byte_to_opcode(&mut self, byte: u8) -> Result<OpCode, ParserError> {
        let opcode = (byte & 0xf0) >> 4;

        match opcode {
            0x00 => Ok(OpCode::NOP),
            0x06 => Ok(OpCode::NOT),
            0x0f => Ok(OpCode::HLT),
            _ => match self.next() {
                Some(operand) => match opcode {
                    0x01 => Ok(OpCode::STA(operand)),
                    0x02 => Ok(OpCode::LDA(operand)),
                    0x03 => Ok(OpCode::ADD(operand)),
                    0x04 => Ok(OpCode::OR(operand)),
                    0x05 => Ok(OpCode::AND(operand)),
                    0x08 => Ok(OpCode::JMP(operand)),
                    0x09 => Ok(OpCode::JN(operand)),
                    0x0a => Ok(OpCode::JZ(operand)),
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
