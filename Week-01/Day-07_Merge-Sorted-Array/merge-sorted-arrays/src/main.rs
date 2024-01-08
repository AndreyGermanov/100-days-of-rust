
fn merge_sorted_arrays(nums1: &[i32], nums2: &[i32], m:usize, n:usize) -> Vec<i32> {
    if m < n || n == 0 {
        return Vec::from(&nums1[0..m]);
    }
    let mut result = Vec::new();
    let mut nums1_iter = nums1.iter().take(m).peekable();
    let mut nums2_iter = nums2.iter().take(n).peekable();
    while (nums1_iter.peek(), nums2_iter.peek()) != (None, None) {
        match (nums1_iter.peek(), nums2_iter.peek()) {
            (Some(_), None) => result.push(*nums1_iter.next().unwrap()),
            (None, Some(_)) => result.push(*nums2_iter.next().unwrap()),
            (Some(x), Some(y)) if x > y => result.push(*nums2_iter.next().unwrap()),
            (Some(_), Some(_))  => result.push(*nums1_iter.next().unwrap()),
            _ => break
        }
    }
    return result;
}

#[test]
fn test_merge_sorted_arrays() {
    assert_eq!(merge_sorted_arrays(&[1,2,3,0,0,0], &[2,5,6], 3, 3),[1,2,2,3,5,6]);
    assert_eq!(merge_sorted_arrays(&[1,4,5,6,7,8], &[2,3,4],3, 2), [1,2,3,4,5]);
    assert_eq!(merge_sorted_arrays(&[1,4,5,6,8,9], &[4,8,9],3, 0),[1,4,5]);
    assert_eq!(merge_sorted_arrays(&[1,4,5,6,8,9], &[4,8,9], 0, 0), []);
    assert_eq!(merge_sorted_arrays(&[1,4,5,6,8,9], &[4,8,9], 0, 3), []);
}
