trait Instruction {
    fn operand(&self) -> Option<u8>;
}

#[derive(Debug)]
pub enum OpCode {
    NOP,
    STA(u8),
    LDA(u8),
    ADD(u8),
    OR(u8),
    AND(u8),
    NOT,
    JMP(u8),
    JN(u8),
    JZ(u8),
    HLT,
}

impl Instruction for OpCode {
    fn operand(&self) -> Option<u8> {
        match *self {
            OpCode::NOP | OpCode::NOT | OpCode::HLT => None,
            OpCode::STA(operand) => Some(operand),
            OpCode::LDA(operand) => Some(operand),
            OpCode::ADD(operand) => Some(operand),
            OpCode::OR(operand) => Some(operand),
            OpCode::AND(operand) => Some(operand),
            OpCode::JMP(operand) => Some(operand),
            OpCode::JN(operand) => Some(operand),
            OpCode::JZ(operand) => Some(operand),
        }
    }
}
