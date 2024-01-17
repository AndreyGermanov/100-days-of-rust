fn nim(n: u16) -> bool {
    if n % 4 == 0 { false } else { true }
}

#[test]
fn test_nim() {
    assert_eq!(nim(1), true);
    assert_eq!(nim(2), true);
    assert_eq!(nim(3), true);
    assert_eq!(nim(4), false);
    assert_eq!(nim(5), true);
    assert_eq!(nim(6), true);
    assert_eq!(nim(7), true);
    assert_eq!(nim(8), false);
    assert_eq!(nim(9), true);
    assert_eq!(nim(10), true);
    assert_eq!(nim(11), true);
    assert_eq!(nim(12), false);
}
