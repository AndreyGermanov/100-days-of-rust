use regex;

fn min_turns<'a>(src: &'a str, dest: &'a str) -> Result<u32, &'a str> {
    let digits: Vec<u32> = prepare(src, dest)?;
    Ok((&digits[0..4]).iter().enumerate().map(|(index, src_dig)| {
        let diff = src_dig.abs_diff(digits[index + 4]);
        if diff > 5 { 10 - diff } else { diff }
    }).sum())
}

fn prepare<'a>(src: &'a str, dest: &'a str) -> Result<Vec<u32>, &'a str> {
    let re = regex::Regex::new("[0-9]+").unwrap();
    if src.len() != 4 || dest.len() != 4 || !re.is_match(src) || !re.is_match(dest) {
        return Err("Incorrect input");
    }
    Ok(format!("{}{}",src,dest).chars().map(|ch| { ch.to_digit(10).unwrap()}).collect())
}

#[test]
fn test_min_turns() {
    assert_eq!(min_turns("4089","5672").unwrap(), 9);
    assert_eq!(min_turns("1111", "1100").unwrap(), 2);
    assert_eq!(min_turns("2391", "4984").unwrap(), 10);
    assert_eq!(min_turns("1111", "1111").unwrap(), 0);
    assert_eq!(min_turns("9876","1111").unwrap(), 14);
    assert!(min_turns("389","4523").is_err());
    assert!(min_turns("","").is_err());
    assert!(min_turns("534543534","34645654").is_err());
    assert!(min_turns("lock","unlock").is_err());
}
