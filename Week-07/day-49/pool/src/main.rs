use ndarray::{Array2, arr2};

fn is_legitimate(pool: Array2<u8>) -> bool {
    pool.row(0).to_vec().iter().sum::<u8>()+
    pool.row(pool.shape()[0]-1).iter().sum::<u8>()+
    pool.column(0).iter().sum::<u8>()+
    pool.column(pool.shape()[1]-1).iter().sum::<u8>() == 0
}

#[test]
fn test_is_legitimate() {
    assert_eq!(is_legitimate(arr2(
        &[[0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 0],
        [0, 1, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0]])),true);
    assert_eq!(is_legitimate(arr2(
        &[[1, 1, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 1, 1, 0, 0],
        [0, 1, 1, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0]])),false);
    assert_eq!(is_legitimate(arr2(
        &[[0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 1, 1, 0, 0, 0],
            [0, 1, 1, 1, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0]])),true);
    assert_eq!(is_legitimate(arr2(
        &[[0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 1, 1, 0, 0, 0],
            [0, 1, 1, 1, 1, 1, 0, 0],
            [0, 0, 1, 1, 1, 0, 0, 0]])),false);
    assert_eq!(is_legitimate(arr2(
        &[[0, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 0, 0]])),true);
}
