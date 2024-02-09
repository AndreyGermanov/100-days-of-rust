fn how_many_games(p: u8, d:u8, m: u8, s: u8) -> u8 {
    let mut sum: u8 = 0;
    let mut result: u8 = 0;
    let mut p = p;
    loop {
        sum += p;
        p = m.max(p-d);
        result += 1;
        if sum + p > s { break result }
    }
}

#[test]
fn test_how_many_games() {
    assert_eq!(how_many_games(20,3,6,80), 6);
    assert_eq!(how_many_games(20,3,6,85), 7);
    assert_eq!(how_many_games(20,3,6,60), 3);
}