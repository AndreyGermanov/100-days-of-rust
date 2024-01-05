fn progress_days(days: &Vec<u32>) -> u32 {
    let mut prev_item:Option<&u32> = None;
    days.iter().map(|item| {
        let progress = prev_item.is_some() && item > prev_item.unwrap();
        prev_item.replace(item);
        progress as u32
    }).sum()
}

#[test]
fn test_progress_days() {
    assert_eq!(progress_days(&vec![3, 4, 1, 2]), 2);
    assert_eq!(progress_days(&vec![10, 11, 12, 9, 10]), 3);
    assert_eq!(progress_days(&vec![6, 5, 4, 3, 2, 9]), 1);
    assert_eq!(progress_days(&vec![9, 9]), 0);
    assert_eq!(progress_days(&vec![]), 0);
    assert_eq!(progress_days(&vec![1, 1, 1, 1, 4, 3, 2, 1, 0, 5]), 2);
    assert_eq!(progress_days(&vec![0, 0, 1, 2, 1]), 2);
}