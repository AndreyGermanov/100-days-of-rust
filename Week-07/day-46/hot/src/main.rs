fn prevent_distractions(input: &str) -> &str {
    let stop_list = ["anime", "meme", "vines", "roasts", "Danny DeVito"];
    for word in stop_list {
        if input.to_lowercase().contains(&word.to_lowercase()) {
            return "NO!"
        }
    }
    return "Safe watching!"
}

#[test]
fn test_prevent_distractions() {
    assert_eq!(prevent_distractions("vines that butter my eggroll"), "NO!");
    assert_eq!(prevent_distractions("Hot pictures of Danny DeVito"),"NO!");
    assert_eq!(prevent_distractions("How to ace BC Calculus in 5 Easy Steps"), "Safe watching!");
}
