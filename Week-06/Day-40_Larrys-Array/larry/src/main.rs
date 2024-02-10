fn able_to_sort(input: &[u8]) -> bool {
    (0..input.len()).fold(0,|mut accum,idx1| {
        for idx2 in idx1..input.len() { if input[idx1] > input[idx2] { accum +=1 } }
        accum
    }) % 2 == 0
}

#[test]
fn test_larry() {
    assert_eq!(able_to_sort(&[1, 6, 5, 2, 4, 3]), true);
    assert_eq!(able_to_sort(&[3, 1, 2]), true);
    assert_eq!(able_to_sort(&[1, 3, 4, 2]), true);
    assert_eq!(able_to_sort(&[1, 2, 3, 5, 4]), false);
}
