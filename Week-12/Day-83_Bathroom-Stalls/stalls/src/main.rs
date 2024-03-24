fn check_stalls(n: i32, k:i32) -> (i32, i32) {
    let mut result = occupy(n);
    let mut k = k;
    while k > 1 {
        if k % 2 == 0 {
            result = occupy(result.1);
            k /= 2
        } else {
            result = occupy(result.0);
            k = (k - 1) / 2

        }
    }
    return if result.1 < result.1 { (result.0,result.1)} else { (result.1, result.0) }
}

fn occupy(n: i32) -> (i32, i32) {
    return if n % 2 == 0 { (n/2-1, n/2) } else { ((n-1)/2, (n-1)/2) }
}

#[test]
fn test_check_stalls() {
    assert_eq!(check_stalls(4,2),(1,0));
    assert_eq!(check_stalls(5,2),(1,0));
    assert_eq!(check_stalls(6,2),(1,1));
    assert_eq!(check_stalls(1000,1000),(0,0));
    assert_eq!(check_stalls(1000,1),(500,499));
}


