use super::*;

#[test]
fn test_next() {
    let file = File::open("binary_files/file1.dat").unwrap();
    let mut parser = Parser::new(file);

    assert_eq!(parser.next(), Some(0x61));
}

#[test]
fn byte_to_opcode_when_nop() {
    let file = File::open("binary_files/nop.dat").unwrap();
    let mut parser = Parser::new(file);

    let opcode = parser.byte_to_opcode()
        .expect("should return NOP opcode");

    match opcode {
        OpCode::NOP => (),
        _ => panic!("should be NOP OpCode"),
    };
}

#[test]
fn byte_to_opcode_when_sta() {
    let file = File::open("binary_files/sta.dat").unwrap();
    let mut parser = Parser::new(file);

    let opcode = parser.byte_to_opcode()
        .expect("should return STA opcode");

    match opcode {
        OpCode::STA(operand) => (),
        _ => panic!("should be STA OpCode"),
    };
}
