fn withdraw(amount: &mut u64) -> u64 {
    [100,20,10,5,1].iter().fold(0, |result,item| {
        let count = *amount / *item;
        if count == 0 { return result }
        *amount -= item * count;
        result+count
    })
}

#[test]
fn test_withdraw() {
    assert_eq!(withdraw(&mut 0), 0);
    assert_eq!(withdraw(&mut 1), 1);
    assert_eq!(withdraw(&mut 15), 2);
    assert_eq!(withdraw(&mut 35), 3);
    assert_eq!(withdraw(&mut 43),5);
    assert_eq!(withdraw(&mut 125),3);
    assert_eq!(withdraw(&mut 1000000000),10000000);
}


