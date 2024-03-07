use rand::Rng;

fn generate_isbn() -> String {
    let mut rng = rand::thread_rng();
    loop {
        let (mut isbn, sum) = (0..9)
            .enumerate()
            .fold(("".to_string(), 0), |(isbn, mut sum), (idx, _)| {
            let number = rng.gen_range(0..10);
            sum += number * (10 - idx);
            (format!("{}{}", number,isbn), sum)
        });
        isbn = if sum%11==0 {format!("{}{}", isbn, 11-(sum % 11))} else {format!("{}{}",isbn,"X")};
        if is_valid_isbn(isbn.as_str()) { break isbn }
    }
}

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
fn test_generate_isbn() {
    for _ in 0..10 {
        let isbn = generate_isbn();
        println!("{}", isbn);
        assert!(is_valid_isbn(isbn.as_str()));
    }
}




