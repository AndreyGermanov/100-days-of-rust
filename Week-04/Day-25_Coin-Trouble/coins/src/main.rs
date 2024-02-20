fn can_distribute(coins: &mut Vec<u8>) -> bool {
    if coins.len() < 3 || coins.iter().sum::<u8>() % 3 != 0 {
        return false;
    }
    let target = coins.iter().sum::<u8>() / 3;

    for _ in 0..3 {
        let mut tmp = vec![];
        for idx in 0..coins.len() {
            if coins[idx] <= target {
                tmp.push(coins[idx])
            }
        }
        tmp.sort();
        let mut sum = 0;
        let mut idx = tmp.len()-1;
        loop {
            if tmp[idx] + sum <= target {
                sum += tmp[idx];
                let pos = coins.iter().position(|item| *item == tmp[idx]);
                if pos.is_some() {
                    coins.remove(pos.unwrap());
                }
            }
            if idx == 0 { break; }
            idx -= 1;
        }

        if sum != target {
            return false;
        }
    }
    return true;
}

#[test]
fn test_can_distribute() {
    assert_eq!(can_distribute(&mut vec![1, 2, 3, 2, 2, 2, 3]), true);
    assert_eq!(can_distribute(&mut vec![5, 3, 10, 1, 2]), false);
    assert_eq!(can_distribute(&mut vec![2, 4, 3, 2, 4, 9, 7, 8, 6, 9]), true);
    assert_eq!(can_distribute(&mut vec![1, 1, 1, 1, 1, 1]), true);
    assert_eq!(can_distribute(&mut vec![10,1,1]), false);
    assert_eq!(can_distribute(&mut vec![10]), false);
    assert_eq!(can_distribute(&mut vec![10,5]), false);
    assert_eq!(can_distribute(&mut vec![]), false);
}
