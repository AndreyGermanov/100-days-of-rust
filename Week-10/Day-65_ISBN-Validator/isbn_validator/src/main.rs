fn is_valid_isbn(input: &str) -> bool {
    let (mut sum, mut count,mut counter) = (0, 0 ,10);
    for ch in input.chars() {
        if counter == 0 { return false }
        if ch.is_numeric() || (ch == 'X' && count == 9) {
            sum += if ch == 'X' { counter * 10 } else {counter * ch.to_digit(10).unwrap()};
            counter -=1;
            count += 1;
        }
    }
    count == 10 && sum % 11 == 0
}

#[test]
fn test_is_valid_isbn() {
    assert_eq!(is_valid_isbn("0-7475-3269-9"),true);
    assert_eq!(is_valid_isbn("156881111X"),true);
    assert_eq!(is_valid_isbn("0-7475-3269-91"),false);
    assert_eq!(is_valid_isbn("--7475-3269-9"),false);
    assert_eq!(is_valid_isbn("X568811115"),false);
}