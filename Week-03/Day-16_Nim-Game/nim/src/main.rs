fn do_i_win_num(n: u16) -> bool {
    if n % 4 == 0 { false } else { true }
}

#[test]
fn test_do_i_win_nim() {
    assert_eq!(do_i_win_num(1), true);
    assert_eq!(do_i_win_num(2), true);
    assert_eq!(do_i_win_num(3), true);
    assert_eq!(do_i_win_num(4), false);
    assert_eq!(do_i_win_num(5), true);
    assert_eq!(do_i_win_num(6), true);
    assert_eq!(do_i_win_num(7), true);
    assert_eq!(do_i_win_num(8), false);
    assert_eq!(do_i_win_num(9), true);
    assert_eq!(do_i_win_num(10), true);
    assert_eq!(do_i_win_num(11), true);
    assert_eq!(do_i_win_num(12), false);
}
