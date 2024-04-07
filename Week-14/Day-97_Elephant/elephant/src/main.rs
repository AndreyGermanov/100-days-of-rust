fn count_moves(dest: usize) -> usize {
    let mut dest = dest;
    let mut result = 0;
    loop {
        result = [5,4,3,2,1].iter().fold(result,|result, item| {
            if *item > dest { return result }
            dest -= item;
            result + 1
        });
        if dest <= 0 {
            break result
        }
    }
}

#[test]
fn test_count_moves() {
    assert_eq!(count_moves(5),1);
    assert_eq!(count_moves(12),3);
}


