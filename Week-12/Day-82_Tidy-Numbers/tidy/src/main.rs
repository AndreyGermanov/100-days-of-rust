use std::iter;

fn last_number(num: usize) -> usize {
    if num < 10 { return num }
    let mut digits:Vec<_> = format!("{}",num).chars()
        .map(|item| item.to_digit(10).unwrap()).collect();
    let mut index = digits.len()-2;
    while index >= 0 {
        if digits[index] > digits[index+1] {
            if digits[index+1] == 0 {
                digits = iter::repeat(9).take(digits.len()-1).collect()
            } else {
                digits[index] -= 1;
                for idx in index + 1..digits.len() { digits[idx] = 9 }
            }
            return digits.iter()
                .map(|digit| digit.to_string())
                .collect::<Vec<_>>().join("")
                .parse::<usize>().unwrap();
        }
        index -=1
    }
    return num;
}

#[test]
fn test_last_number() {
    assert_eq!(last_number(132), 129);
    assert_eq!(last_number(1000), 999);
    assert_eq!(last_number(7),7);
    assert_eq!(last_number(111111111111111110), 99999999999999999);
}


