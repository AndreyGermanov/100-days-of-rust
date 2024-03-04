fn calc_months(input: [u64;3]) -> u32 {
    let (males, females, target) = (input[0], input[1], input[2]);
    let (mut total,mut months,mut all) = (females+males,0,vec![(0,0);96]);
    all[0] = (females, males);
    loop {
        let new = (0..92).enumerate().fold((0,0),|accum,(idx,_)| {
            (accum.0 + all[idx+4].0 * 9, accum.1 + all[idx+4].0 * 5)
        });
        total = total + new.0 + new.1 - all[95].0 + all[95].1;
        if total >= target { break months - 1 }
        all.remove(95);
        all.insert(0,new);
        months += 1
    }
}

#[test]
fn test_calc_months() {
    assert_eq!(calc_months([2,4,1000000000]),32);
    assert_eq!(calc_months([2,4,15000000000]),36);
}
