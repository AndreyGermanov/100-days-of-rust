fn calc_hours(candles: &mut usize, join_candles: usize) -> usize {
    if join_candles < 2 { return *candles }
    let mut result = 0;
    while *candles > 0 {
        result += *candles;
        *candles /= join_candles;
    }
    return result
}

#[test]
fn test_calc_hours() {
    assert_eq!(calc_hours(&mut 4,0), 4);
    assert_eq!(calc_hours(&mut 4,1), 4);
    assert_eq!(calc_hours(&mut 4,2), 7);
    assert_eq!(calc_hours(&mut 6,3), 8);
}