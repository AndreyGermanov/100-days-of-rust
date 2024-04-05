fn num_cashes(times:Vec<(usize,usize)>) -> usize {
    times.iter().skip(1).fold((1,1,times[0]),|(result,count,prev), item| {
        if item.0 == prev.0 && item.1 == prev.1 {
            (if count+1 > result {count+1} else {result},count+1,*item)
        } else {
            (result, 1, *item)
        }
    }).0
}

#[test]
fn test_num_cashes() {
    assert_eq!(num_cashes(vec![(8,0),(8,10),(8,10),(8,45)]),2);
    assert_eq!(num_cashes(vec![(0,12),(10,11),(22,22)]),1);
}
