fn can_fit(weights:&mut Vec<u8>, num_bags: usize) -> bool {
    let mut bags:Vec<u8> = vec![0;num_bags];
    let mut weight = 0;
    while weight == 0 && !weights.is_empty() {
        weight = weights[weights.len()-1];
        for index in 0..bags.len() {
            if bags[index] + weight <= 10 {
                bags[index] = bags[index] + weight;
                if bags[index] == 10 {
                    bags.remove(index);
                }
                weights.pop();
                weight = 0;
                break;
            }
        }
    }
    return weights.is_empty() && num_bags > 0
}

#[test]
fn test_can_fit() {
    assert_eq!(can_fit(&mut vec![],0), false);
    assert_eq!(can_fit(&mut vec![],3), true);
    assert_eq!(can_fit(&mut vec![2, 1, 2, 5, 4, 3, 6, 1, 1, 9, 3, 2],4), true);
    assert_eq!(can_fit(&mut vec![2, 7, 1, 3, 3, 4, 7, 4, 1, 8, 2],4), false);
    assert_eq!(can_fit(&mut vec![2, 1, 2, 5, 4, 3, 6, 1, 1, 19, 3, 2],4), false);
    assert_eq!(can_fit(&mut vec![9, 9, 9],2), false);
    assert_eq!(can_fit(&mut vec![15],1), false);
}