fn calc_weeks(num_people: i32, num_fruits: i32) -> i32 {
    let (mut num_people, mut num_fruits, mut weeks) = (num_people, num_fruits, 1);
    let mut _fruits = 0;
    loop {
        _fruits = num_fruits + _fruits;
        num_people  -= _fruits;
        _fruits = 0;
        _fruits = num_fruits * weeks;
        num_fruits = num_fruits * 2;
        weeks +=1;
        if num_people <= 0 { break weeks;}
    }
}

#[test]
fn test_calc_weeks() {
    assert_eq!(calc_weeks(200, 15), 5);
    assert_eq!(calc_weeks(50000, 1), 14);
    assert_eq!(calc_weeks(150000, 250), 9);
}

