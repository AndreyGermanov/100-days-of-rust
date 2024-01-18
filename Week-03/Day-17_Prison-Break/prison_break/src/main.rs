fn prison_break(cells: &[u8]) -> u8 {
    let mut inversed = false;
    if cells.len() == 0 || cells[0] == 0 { return 0; }
    cells.iter().skip(1).fold(0, |mut accum:u8, cell| {
        if (*cell == 1 && !inversed) || (*cell == 0 && inversed) {
            accum += 1;
            inversed = !inversed;
        }
        return accum;
    })
}

#[test]
fn test_prison_break() {
    assert_eq!(prison_break(&[]), 0);
    assert_eq!(prison_break(&[1]), 0);
    assert_eq!(prison_break(&[1, 0]), 0);
    assert_eq!(prison_break(&[1, 1]), 1);
    assert_eq!(prison_break(&[0, 1, 1, 1]), 0);
    assert_eq!(prison_break(&[0, 0, 0]), 0);
    assert_eq!(prison_break(&[1, 1, 1]), 1);
    assert_eq!(prison_break(&[1, 1, 0, 0, 0, 1, 0]), 4);
}