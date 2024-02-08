fn breaking_records(scores: &[u8]) -> (u8,u8) {
    if scores.is_empty() { return (0,0); }
    let (mut min, mut max) = (scores[0], scores[0]);
    scores.iter().skip(1).fold((0, 0), |mut accum, value| {
        if *value < min { accum.1 += 1; min = *value }
        if *value > max { accum.0 += 1; max = *value }
        accum
    })
}

#[test]
fn test_breaking_records() {
    assert_eq!(breaking_records(&[10,5,20,20,4,5,2,25,1]), (2, 4));
    assert_eq!(breaking_records(&[3,4,21,36,10,28,35,5,24,42]), (4, 0));
    assert_eq!(breaking_records(&[]), (0, 0));
    assert_eq!(breaking_records(&[1]), (0, 0));
    assert_eq!(breaking_records(&[10,5]), (0, 1));
}
