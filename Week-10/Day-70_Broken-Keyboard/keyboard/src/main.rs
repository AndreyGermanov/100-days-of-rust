use std::fs::read_to_string;

fn longest_word(input: &str) -> String {
    let chars:Vec<_> = input.chars().collect();
    read_to_string("dictionary1.txt").unwrap().lines().filter(|line| {
        for ch in line.chars() {
            if !chars.contains(&ch) { return false }
        }
        true
    }).fold("".to_string(),|mut result,line| {
        if line.len() > result.len() { result = line.to_string() }
        result
    })
}

#[test]
fn test_longest_word() {
    assert_eq!(longest_word("abcd"), "abaca");
    assert_eq!(longest_word("edcf"), "deeded");
    assert_eq!(longest_word("bnik"), "bikini");
    assert_eq!(longest_word("poil"), "lollipop");
    assert_eq!(longest_word("vybu"), "bubby");
    assert_eq!(longest_word("subtoxymerhlac"), "carboxymethylcelluloses");
}