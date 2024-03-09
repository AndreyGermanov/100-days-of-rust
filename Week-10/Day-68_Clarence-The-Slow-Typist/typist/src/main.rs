fn type_ip(ip: &str) -> f64 {
    let sqrt2 = f64::sqrt(2.0);
    let distances = [
        [0.0,sqrt2+2.0,3.0,sqrt2+2.0,sqrt2+1.0,2.0,sqrt2+1.0,sqrt2,1.0,sqrt2],
        [sqrt2+2.0,0.0,1.0,2.0,1.0,sqrt2,sqrt2+1.0,2.0,sqrt2+1.0,2.0*sqrt2],
        [3.0,1.0,0.0,1.0,sqrt2,1.0,sqrt2,sqrt2+1.0,2.0,sqrt2+1.0],
        [sqrt2+2.0,2.0,1.0,0.0,sqrt2+1.0,sqrt2,1.0,2.0*sqrt2,sqrt2+1.0,2.0],
        [sqrt2+1.0,1.0,sqrt2,sqrt2+1.0,0.0,1.0,2.0,1.0,sqrt2,sqrt2+1.0],
        [2.0,sqrt2,1.0,sqrt2,1.0,0.0,1.0,sqrt2,1.0,sqrt2],
        [sqrt2,sqrt2+1.0,sqrt2,1.0,2.0,1.0,0.0,sqrt2+1.0,sqrt2,1.0],
        [sqrt2,2.0,sqrt2+1.0,2.0*sqrt2,1.0,sqrt2,sqrt2+1.0,0.0,1.0,2.0],
        [1.0,sqrt2+1.0,2.0,sqrt2+1.0,sqrt2,1.0,sqrt2,1.0,0.0,1.0],
        [sqrt2,2.0*sqrt2,sqrt2+1.0,2.0,sqrt2+1.0,sqrt2,1.0,2.0,1.0,0.0],
    ];
    let result = ip.chars()
        .filter(|ch| ch.is_numeric())
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .enumerate()
        .fold((0,0.0f64),|(prev_item,sum),(index,item)| {
            if index == 0 { (item,0.0) } else { (item, sum + distances[prev_item][item]) }
        }).1;
    (result * 100.0f64).round() / 100.0
}

#[test]
fn test_type_ip() {
    assert_eq!(type_ip("1.1.1.1"), 0.0);
    assert_eq!(type_ip("1.1.1.2"), 1.0);
    assert_eq!(type_ip("219.45.143.143"), 17.49);
    assert_eq!(type_ip("7.8.5.1"), 3.41);
    assert_eq!(type_ip("123.654.7.89"), 8.00);
}

