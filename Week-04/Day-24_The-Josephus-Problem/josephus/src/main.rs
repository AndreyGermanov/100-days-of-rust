fn josephus(num: u8, interval: u8) -> u8 {
    if num == 1 { 1 } else { (josephus(num - 1, interval) + interval - 1) % num + 1 }
}

#[test]
fn test_josephus() {
    assert_eq!(josephus(41, 3), 31);
    assert_eq!(josephus(35, 11), 18);
    assert_eq!(josephus(11, 1), 11);
    assert_eq!(josephus(2, 2), 1);
}
