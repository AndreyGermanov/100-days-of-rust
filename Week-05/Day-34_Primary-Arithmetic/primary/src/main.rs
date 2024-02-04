fn carry_operations(mut num1: u32, mut num2: u32) -> u32 {
    let mut result = 0;
    loop {
        let num2_digit = num2 % 10;
        num2 = (num2 - num2_digit) / 10;
        let num1_digit = num1 % 10 + num2_digit;
        num1 = (num1 + num1_digit) / 10;
        if (num1_digit) > 9 { result += 1; }
        if num2 <= 0 { break result; }
    }
}

#[test]
fn test_carry_operations() {
    assert_eq!(carry_operations(123, 456), 0);
    assert_eq!(carry_operations(555, 555), 3);
    assert_eq!(carry_operations(123, 594), 1);
    assert_eq!(carry_operations(555, 545), 3);
    assert_eq!(carry_operations(1, 20000), 0);
    assert_eq!(carry_operations(1, 2), 0);
    assert_eq!(carry_operations(1, 9), 1);
    assert_eq!(carry_operations(8, 1), 0);
}
