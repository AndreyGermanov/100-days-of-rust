fn keyboard_mistake_fix(input: &str) -> String {
    let querty:Vec<char> = "`1234567890-=QWERTYUIOP[]\\ASDFGHJKL;'ZXCVBNM,./".chars().collect();
    input.chars().map(|ch| {
        match querty.iter().position(|res_ch| *res_ch == ch) {
            None => ch,
            Some(pos) if pos>=1 && pos<querty.len() => *querty.iter().nth(pos-1).unwrap(),
            Some(_) => ch
        }
    }).collect()
}

#[test]
fn test_keyboard_mistake_fix() {
    assert_eq!(keyboard_mistake_fix("O S, GOMR YPFSU/"), "I AM FINE TODAY.");
}