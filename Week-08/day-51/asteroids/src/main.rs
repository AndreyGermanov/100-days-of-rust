fn collide(input: &mut Vec<i8>) -> &[i8] {
    loop {
        let mut index = 0;
        let mut changed = false;
        while input.len() > 0 && index < input.len() - 1 {
            if !(input[index] > 0 && input[index+1] < 0) { index += 1; continue }
            if i8::abs(input[index]) > i8::abs(input[index + 1]) {
                input.remove(index + 1);
            } else if i8::abs(input[index]) < i8::abs(input[index + 1]) {
                input.remove(index);
            } else {
                input.remove(index);
                input.remove(index);
            }
            changed = true;
        }
        if !changed { break input }
    }
}

#[test]
fn test_collide() {
    assert_eq!(collide(&mut vec![5,10,-5]), &[5,10]);
    assert_eq!(collide(&mut vec![8,-8]), &[]);
    assert_eq!(collide(&mut vec![10,2,-5]), &[10]);
    assert_eq!(collide(&mut vec![-2,-1,1,2]), &[-2,-1,1,2]);
    assert_eq!(collide(&mut vec![]), &[]);
    assert_eq!(collide(&mut vec![1]), &[1]);
}
