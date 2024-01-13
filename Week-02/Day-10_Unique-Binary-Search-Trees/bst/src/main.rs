fn bst_num(n: i32) -> i32 {
    let numerator = (2..=n).fold(1, |accum, k| accum * (n + k));
    let denominator = (2..=n).fold(1, |accum, k| accum * k);
    numerator / denominator
}

#[test]
fn test_bst_num() {
    assert_eq!(bst_num(3), 5);
    assert_eq!(bst_num(1), 1);
}
