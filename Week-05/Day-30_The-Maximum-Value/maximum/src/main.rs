fn get_biggest_number(number: i64, digit: u64) -> i64 {
    if number == 0 { return digit as i64 * 10 };
    let zeros = get_trailing_zeros_count(number);
    let negate = if number > 0 { 1 } else { -1 };
    let mut number = invert_number(number * negate) as u64;
    let mut multiplier = 1;
    let mut matched = false;
    let mut result = 0;
    while number > 0 {
        let cur_digit = number % 10;
        if (digit >= cur_digit && negate == 1 || digit < cur_digit && negate == -1) && !matched {
            result += digit * multiplier;
            multiplier *= 10;
            matched = true;
        }
        result += cur_digit * multiplier;
        multiplier *= 10;
        number = (number - cur_digit) / 10;
    }
    if !matched { result += digit * multiplier; }
    return invert_number(result as i64) * negate * (if zeros > 0 {zeros as i64 * 10} else { 1 });
}

#[test]
fn test_get_biggest_number() {
    assert_eq!(get_biggest_number(0,0),0);
    assert_eq!(get_biggest_number(0,3),30);
    assert_eq!(get_biggest_number(276,3),3276);
    assert_eq!(get_biggest_number(860,7),8760);
    assert_eq!(get_biggest_number(999,4), 9994);
    assert_eq!(get_biggest_number(-999,4), -4999);
    assert_eq!(get_biggest_number(-5,3), -35);
    assert_eq!(get_biggest_number(-345,9),-3459);
    assert_eq!(get_biggest_number(-345,1),-1345);
    assert_eq!(get_biggest_number(457547567,5),5457547567);
    assert_eq!(get_biggest_number(-457547567,5),-4557547567);
}

fn invert_number(number: i64) -> i64 {
    let mut mult = 1;
    if number < 0 { mult = -1; }
    let mut number = number * mult;
    let mut result = 0;
    while number>0 {
        result = result * 10 + number % 10;
        number = number / 10;
    }
    return result * mult;
}

#[test]
fn test_invert_number() {
    assert_eq!(invert_number(123), 321);
    assert_eq!(invert_number(-123), -321);
    assert_eq!(invert_number(453234), 432354);
}

fn get_trailing_zeros_count(number: i64) -> u32 {
    let mut zeros = 0;
    let mut num_for_zeros = number;
    while num_for_zeros % 10 == 0 {
        zeros += 1;
        num_for_zeros /= 10;
    }
    return zeros;
}