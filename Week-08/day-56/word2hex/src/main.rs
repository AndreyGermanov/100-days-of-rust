fn word2hex(word: &str) -> String {
    word.chars().fold("".to_string(),|hex,ch| {
        format!("{}{:0>2x} ",hex, ch as u8)
    }).trim().to_string()
}

#[test]
fn test_word2hex() {
    assert_eq!(word2hex("hello world"),"68 65 6c 6c 6f 20 77 6f 72 6c 64");
    assert_eq!(word2hex("Big Boi"),"42 69 67 20 42 6f 69");
    assert_eq!(word2hex("Marty Poppinson"),"4d 61 72 74 79 20 50 6f 70 70 69 6e 73 6f 6e");
}

