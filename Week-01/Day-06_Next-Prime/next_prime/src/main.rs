fn next_prime(number:i32) -> i32 {
    for item in number..i32::MAX {
        if is_prime(item) {
            return item
        }
    }
    return 0
}

fn is_prime(number: i32) -> bool {
    if number < 2 {
        return false;
    }
    let mut next_number = 2;
    while next_number * next_number <= number {
        if number % next_number == 0 {
            return false;
        }
        next_number += 1;
    }
    return true;
}

#[test]
fn test_next_prime() {
    assert_eq!(next_prime(0), 2);
    assert_eq!(next_prime(1), 2);
    assert_eq!(next_prime(2), 2);
    assert_eq!(next_prime(3), 3);
    assert_eq!(next_prime(4), 5);
    assert_eq!(next_prime(5), 5);
    assert_eq!(next_prime(12), 13);
    assert_eq!(next_prime(24), 29);
    assert_eq!(next_prime(11), 11);
    assert_eq!(next_prime(9050), 9059);
}
