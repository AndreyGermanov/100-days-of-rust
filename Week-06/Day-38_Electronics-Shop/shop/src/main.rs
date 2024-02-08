fn get_money_spent(keyboards: &[i8], drives: &[i8], budget:i8 ) -> i8 {
    keyboards.iter().fold(-1_i8,| maximum,item1 | {
        drives.iter().map(|item2| item1+item2)
            .filter(|item| *item <= budget)
            .fold( maximum, |maximum,item| {if item>maximum {item} else {maximum}
        })
    })
}

#[test]
fn test_get_money_spent() {
    assert_eq!(get_money_spent(&[40,50,60],&[5,8,12],60), 58);
    assert_eq!(get_money_spent(&[65],&[80], 40), -1);
    assert_eq!(get_money_spent(&[3, 1],&[5, 2, 8], 10), 9);
    assert_eq!(get_money_spent(&[4],&[5], 5), -1);
}


