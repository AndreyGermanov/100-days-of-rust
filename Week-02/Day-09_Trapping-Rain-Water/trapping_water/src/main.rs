fn trap(height: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, height.len() - 1);
    let mut pool_height = 0;
    let mut trapped = 0;

    while left < right {
        pool_height = pool_height.max(height[left].min(height[right]));

        if height[left] <= height[right] {
            trapped += 0.max(pool_height - height[left]);
            left += 1;
        } else {
            trapped += 0.max(pool_height - height[right]);
            right -= 1;
        }
    }
    return trapped;
}

#[test]
fn test_trap() {
    assert_eq!(trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    assert_eq!(trap(vec![4,2,0,3,2,5]), 9);
}
