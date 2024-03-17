fn max_buffel_bag_value(cakes: Vec<(u32,u32)>,capacity:u32) -> u32 {
    let mut capacity = capacity;
    let mut cakes = cakes;
    let mut result = 0;
    loop {
        let (value, max_idx, cap) = cakes.iter().enumerate()
            .fold((0, 0, 0),
          |(max, idx_max,cap), (index, (cost, value))| {
            let amount = value * (capacity / cost);
            return if amount > max {
                (amount, index, (capacity / cost) * cost)
            } else {
                (max, idx_max, cap)
            }
        });
        if value == 0 { break result }
        result += value;
        capacity -= cap;
        cakes.remove(max_idx);
        if cakes.is_empty() { break result }
    }
}

#[test]
fn test_max_buffel_bag_value() {
    assert_eq!(max_buffel_bag_value(vec![(7, 160), (3, 90), (2, 15)],20),555);
    assert_eq!(max_buffel_bag_value(vec![(3, 160), (7, 90), (1, 15)],20),990);
}

