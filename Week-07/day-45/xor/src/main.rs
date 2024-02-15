fn subtract_xor(mut x:i32, mut y:i32) -> i32 {
    x = x ^ y;
    y = x ^ y;
    x = x ^ y;
    x - y
}

#[test]
fn test_subtract_xor() {
    assert_eq!(subtract_xor(10, 41), 31);
    assert_eq!(subtract_xor(69, 420), 351);
    assert_eq!(subtract_xor(12345, 890412), 878067);
    assert_eq!(subtract_xor(2, 1), -1);
}