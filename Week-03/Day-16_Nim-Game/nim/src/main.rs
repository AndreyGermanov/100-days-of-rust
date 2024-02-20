fn will_i_win_nim(n: u16) -> bool {
    if n % 4 == 0 { false } else { true }
}

#[test]
fn test_do_i_win_nim() {
    assert_eq!(will_i_win_nim(1), true);
    assert_eq!(will_i_win_nim(2), true);
    assert_eq!(will_i_win_nim(3), true);
    assert_eq!(will_i_win_nim(4), false);
    assert_eq!(will_i_win_nim(5), true);
    assert_eq!(will_i_win_nim(6), true);
    assert_eq!(will_i_win_nim(7), true);
    assert_eq!(will_i_win_nim(8), false);
    assert_eq!(will_i_win_nim(9), true);
    assert_eq!(will_i_win_nim(10), true);
    assert_eq!(will_i_win_nim(11), true);
    assert_eq!(will_i_win_nim(12), false);
}