fn get_minutes(hour:u32,minute:u32) -> u32 { (23 - hour) * 60 + (60 - minute) }

#[test]
fn test_get_minutes() {
    assert_eq!(get_minutes(23,55),5);
    assert_eq!(get_minutes(23,0),60);
    assert_eq!(get_minutes(0,1),1439);
    assert_eq!(get_minutes(4,20),1180);
    assert_eq!(get_minutes(23,59),1);
}
