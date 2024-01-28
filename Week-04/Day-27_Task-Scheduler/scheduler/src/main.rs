use std::collections::HashMap;

fn schedule(input: &[char], idle: usize) -> usize {
    let mut tasks_map:HashMap<&char,usize> = HashMap::new();
    input.iter().for_each(|item| {
        tasks_map.insert(item, *tasks_map.get(&item).unwrap_or(&0_usize)+1);
    });
    let maximum = tasks_map.values().max().unwrap();
    let last_len = tasks_map.values().filter(|item| *item == maximum).collect::<Vec<_>>().len();
    return idle.max(input.len().max((maximum - 1) * (idle + 1) + last_len));
}

#[test]
fn test_schedule() {
    assert_eq!(schedule(&[' '],2),2);
    assert_eq!(schedule(&[' '],0),1);
    assert_eq!(schedule(&['A','A','A','B','B','B'], 0), 6);
    assert_eq!(schedule(&['A','A','A','B','B','B'], 2), 8);
    assert_eq!(schedule(&['A','A','A','A','A','A','B','C','D','E','F','G'], 2), 16);
}
