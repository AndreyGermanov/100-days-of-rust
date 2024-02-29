use std::collections::HashMap;

fn balanced_bonus(input: &str) -> bool {
    input.chars().fold(HashMap::new(),|mut result, ch| {
        *result.entry(ch).or_insert(0) += 1;
        result
    }).iter().fold((true, 0),|(result,prev_count), (_,value)| {
        (prev_count == 0 || *value == prev_count, *value)
    }).0
}

#[test]
fn test_balanced_bonus() {
    assert_eq!(balanced_bonus(""),true);
    assert_eq!(balanced_bonus(" "),true);
    assert_eq!(balanced_bonus("xxxyyyzzz"),true);
    assert_eq!(balanced_bonus("abccbaabccba"),true);
    assert_eq!(balanced_bonus("xxxyyyzzzz"),false);
    assert_eq!(balanced_bonus("abcdefghijklmnopqrstuvwxyz"),true);
    assert_eq!(balanced_bonus("pqq"),false);
    assert_eq!(balanced_bonus("fdedfdeffeddefeeeefddf"),false);
    assert_eq!(balanced_bonus("www"),true);
    assert_eq!(balanced_bonus("x"),true);
    assert_eq!(balanced_bonus(""),true);
}