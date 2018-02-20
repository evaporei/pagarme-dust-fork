use super::*;

#[test]
fn test_get_file_content() {
    let result = get_file_content(&"test/file.txt".to_string())
        .unwrap();

    assert_eq!("foo\nbar\n", result);
}

#[test]
#[should_panic]
fn test_get_file_content_error() {
    get_file_content(&"test/non_existing_file".to_string())
        .unwrap();
}

#[test]
fn test_concat() {
    let file_names = vec![
        "test/file.txt".to_string(),
        "test/another_file.txt".to_string()
    ];

    let result = concat(&file_names);

    assert_eq!("foo\nbar\nbaz\nqewxbirr\n", result);
}

#[test]
fn test_concat_error() {
    let file_names = vec![
        "test/file.txt".to_string(),
        "test/non_existing_file".to_string()
    ];

    let result = concat(&file_names);

    assert_eq!("foo\nbar\nNo such file or directory (os error 2)", result);
}
