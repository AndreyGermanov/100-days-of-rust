fn can_transform(s: &str, t: &str) -> String {
    let vowels = ['a','e','i','o','u'];
    if s.len() != t.len() { return "No".to_string(); }
    for (index,ch) in s.chars().enumerate() {
        if vowels.contains(&t.chars().nth(index).unwrap()) && !vowels.contains(&ch) {
            return "No".to_string();
        }
        if vowels.contains(&ch) && !vowels.contains(&t.chars().nth(index).unwrap()) {
            return "No".to_string()
        }
    }
    return "Yes".to_string()
}

#[test]
fn test_can_transform() {
    assert_eq!(can_transform("a","u"),"Yes");
    assert_eq!(can_transform("abc","ukm"),"Yes");
    assert_eq!(can_transform("akm","ua"),"No");
}
