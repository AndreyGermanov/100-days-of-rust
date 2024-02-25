fn identical_filter<'a>(input: &'a[&'a str]) -> Vec<&'a str> {
    input.iter().filter(|item| {
        let mut item = item.as_bytes().to_vec();
        item.dedup();
        item.len() == 1
    }).map(|item| *item).collect()
}

#[test]
fn test_identical_filter() {
    assert_eq!(identical_filter(&["aaaaaa", "bc", "d", "eeee", "xyz"]), &["aaaaaa", "d", "eeee"]);
    assert_eq!(identical_filter(&["88", "999", "22", "545", "133"]), &["88", "999", "22"]);
    assert_eq!(identical_filter(&["xxxxo", "oxo", "xox", "ooxxoo", "oxo"]).len(),0);
}
