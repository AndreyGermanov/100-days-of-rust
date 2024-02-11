use std::collections::HashMap;

fn get_num_pairs(input: &[u8]) -> u8 {
    let mut pairs:HashMap<u8, u8> = HashMap::new();
    input.iter().fold(0,|mut accum,item| {
        if pairs.get(item).unwrap_or(&0_u8) > &0_u8 {
            pairs.insert(*item, 0); accum += 1; accum
        } else {
            pairs.insert(*item, 1); accum
        }
    })
}

#[test]
fn test_get_num_pairs() {
    assert_eq!(get_num_pairs(&[1, 2, 1, 2, 1, 3, 2]), 2);
    assert_eq!(get_num_pairs(&[1, 1, 1]), 1);
    assert_eq!(get_num_pairs(&[1]), 0);
    assert_eq!(get_num_pairs(&[]), 0);
    assert_eq!(get_num_pairs(&[3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3]), 5);
}

