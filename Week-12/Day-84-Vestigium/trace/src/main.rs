use ndarray::{arr2, Array2};

fn trace(mat: Array2<u8>) -> (u8,u8,u8) {
    let (mut trace,mut rows, mut cols) = (0,0,0);
    for index in 0.. mat.shape()[0] {
        trace += mat[(index,index)];
        let mut row = mat.row(index).to_vec();row.sort();row.dedup();
        let mut col = mat.column(index).to_vec();col.sort();col.dedup();
        if mat.row(index).to_vec().len() != row.len() { rows += 1; }
        if mat.column(index).to_vec().len() != col.len() { cols += 1; }
    }
    return (trace, rows, cols);

}

#[test]
fn test_trace() {
    assert_eq!(trace(arr2(&[[1,2,3,4], [2,1,4,3], [3,4,1,2], [4,3,2,1]])),(4,0,0));
    assert_eq!(trace(arr2(&[[2,2,2,2], [2,3,2,3], [2,2,2,3], [2,2,2,2]])),(9,4,4));
    assert_eq!(trace(arr2(&[[2,1,3], [1,3,2], [1,2,3]])),(8,0,2));
}

