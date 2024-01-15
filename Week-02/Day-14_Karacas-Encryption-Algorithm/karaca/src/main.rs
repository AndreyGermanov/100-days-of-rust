use std::collections::HashMap;
use std::fmt::Write;

fn do_karaca(input: String) -> String {
    let vowels: HashMap<char, char> = [('a', '0'), ('e', '1'), ('i', '2'), ('o', '2'), ('u','3')]
        .iter().cloned().collect();
    let result: String = input.chars().rev()
        .fold(String::new(),|mut accum,item| {
            write!(&mut accum, "{}", vowels.get(&item).unwrap_or(&item))
                .expect("Could not write");
        accum
    })+"aca";
    result
}

#[test]
fn test_do_karaca() {
    assert_eq!(do_karaca("banana".to_string()), "0n0n0baca");
    assert_eq!(do_karaca("karaca".to_string()), "0c0r0kaca");
    assert_eq!(do_karaca("burak".to_string()), "k0r3baca");
    assert_eq!(do_karaca("alpaca".to_string()), "0c0pl0aca");
}
