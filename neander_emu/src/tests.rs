use super::*;

#[test]
fn test_next() {
    let file = File::open("binary_files/file1.dat").unwrap();
    let mut parser = Parser::new(file);

    assert_eq!(parser.next(), Some(0x61));
}
