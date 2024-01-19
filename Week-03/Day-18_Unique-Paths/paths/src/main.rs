fn unique_paths(m: usize, n: usize) -> usize {
    (n..m + n - 1).fold(1, |accum, item| accum * item / (item - n + 1))
}

#[test]
fn test_unique_paths() {
    assert_eq!(unique_paths(1, 1), 1);
    assert_eq!(unique_paths(3, 2), 3);
    assert_eq!(unique_paths(3, 3), 6);
    assert_eq!(unique_paths(3, 7), 28);
}
