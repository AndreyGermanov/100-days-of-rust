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

fn longest_substring(s: String) -> i32 {
    let mut cur_str = "".to_string();
    s.chars().fold(0,|result, ch| {
        if let Some(pos) = cur_str.chars().position(|chr| ch == chr ) {
            cur_str = cur_str.chars().skip(pos + 1).collect();
        }
        cur_str += ch.to_string().as_str();
        result.max(cur_str.len() as i32)
    })
}

#[test]
fn test_longest_substring() {
    assert_eq!(longest_substring("abcabcbb".to_string()) ,3);
    assert_eq!(longest_substring("bbbbb".to_string()) ,1);
    assert_eq!(longest_substring("pwwkew".to_string()) ,3);
    assert_eq!(longest_substring("".to_string()) ,0);
    assert_eq!(longest_substring(" ".to_string()) ,1);
    assert_eq!(longest_substring("aabaab!bb".to_string()),3)
}