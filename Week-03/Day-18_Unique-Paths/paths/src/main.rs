fn unique_paths(m: usize, n: usize) -> usize {
    let mut result = vec![0; n];
    result[0] = 1;
    for _ in 0..m {
        for col in 1..n {
            result[col] += result[col - 1];
        }
    }
    result[n-1]
}

#[test]
fn test_unique_paths() {
    assert_eq!(unique_paths(1, 1), 1);
    assert_eq!(unique_paths(3, 2), 3);
    assert_eq!(unique_paths(3, 3), 6);
    assert_eq!(unique_paths(3, 7), 28);
}
