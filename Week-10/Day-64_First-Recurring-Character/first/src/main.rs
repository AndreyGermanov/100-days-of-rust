use std::collections::HashSet;

fn first_recurring_char(input: &str) -> Option<char> {
    let mut hash:HashSet<char> = HashSet::new();
    for ch in input.chars() {
        if hash.contains(&ch) { return Some(ch) };
        hash.insert(ch);
    }
    return None;
}

#[test]
fn test_first_recurring_char() {
    assert_eq!(first_recurring_char("ABCDEBC"),Some('B'));
    assert_eq!(first_recurring_char("ABBA"),Some('B'));
    assert_eq!(first_recurring_char("ABCDEF"),None);
    assert_eq!(first_recurring_char(""),None);
}
