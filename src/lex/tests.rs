use super::Lexer;

#[test]
fn test_next_word() {
    let mut l = Lexer::new(r#"hello world
 this# is
a   	 test
"#);
    assert_eq!(l.next_word(), Some(b"hello".to_vec()));
    assert_eq!(l.current_line_number, 1);
    assert_eq!(l.next_word(), Some(b"world".to_vec()));
    assert_eq!(l.current_line_number, 1);
    assert_eq!(l.next_word(), Some(b"\n".to_vec()));
    assert_eq!(l.current_line_number, 2);
    assert_eq!(l.next_word(), Some(b"this".to_vec()));
    assert_eq!(l.current_line_number, 2);
    assert_eq!(l.next_word(), Some(b"\n".to_vec()));
    assert_eq!(l.current_line_number, 3);
    assert_eq!(l.next_word(), Some(b"a".to_vec()));
    assert_eq!(l.current_line_number, 3);
    assert_eq!(l.next_word(), Some(b"test".to_vec()));
    assert_eq!(l.current_line_number, 3);
    assert_eq!(l.next_word(), Some(b"\n".to_vec()));
    assert_eq!(l.current_line_number, 4);
    assert_eq!(l.next_word(), None);
}
