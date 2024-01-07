use std::collections::HashMap;

fn get_num_pairs(line: &str) -> u32 {
    let mut result: u32 = 0;
    let mut pairs:HashMap<char, bool> = HashMap::new();
    line.chars().for_each(|ch| {
        if pairs.contains_key(&ch) && pairs[&ch] == true {
            result += 1;
            pairs.insert(ch, false);
        } else {
            pairs.insert(ch, true);
        }
    });
    return result;
}

#[test]
fn test_get_num_pairs() {
    assert_eq!(get_num_pairs(""), 0);
    assert_eq!(get_num_pairs("A"), 0);
    assert_eq!(get_num_pairs("AA"), 1);
    assert_eq!(get_num_pairs("AAA"), 1);
    assert_eq!(get_num_pairs("ABABC"), 2);
    assert_eq!(get_num_pairs("CABBACCC"), 4);
    assert_eq!(get_num_pairs("ABCDEF"), 0);
}
