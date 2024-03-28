fn do_hulk(num: usize) -> String {
    let actions = &["I hate", "I love"];
    (0..num).fold(vec![],|mut result,index| {
        result.push(actions[index % actions.len()]);
        result
    }).join(" that ").as_str().to_owned() + " it"
}

#[test]
fn test_do_hulk() {
    assert_eq!(do_hulk(1),"I hate it");
    assert_eq!(do_hulk(2),"I hate that I love it");
    assert_eq!(do_hulk(3), "I hate that I love that I hate it");
}
