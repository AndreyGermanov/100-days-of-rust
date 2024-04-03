fn climb_moves(n: usize, m: usize) -> isize {
    if m>n || n <= 0 || n > 10000 || m <= 1 || m > 10 { return - 1 }
    let mut steps = n/2;
    if steps % 2 != 0 { steps += 1 }
    while steps % m != 0 && steps <= n { steps+=1; }
    if steps > n { -1 } else {steps as isize}
}

#[test]
fn test_climb_moves() {
    assert_eq!(climb_moves(10,2), 6);
    assert_eq!(climb_moves(3, 5), -1)
}
