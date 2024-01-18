fn will_i_win_nim(n: u16) -> bool {
    if n % 4 == 0 { false } else { true }
}

use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

// TODO: modify only this function.
fn copy_and_return<'a>(vector: &mut Vec<String>, value: &'a str) -> &'a str {
    vector.push(String::from(value));
    return &value
}

fn main() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}


#[test]
fn test_do_i_win_nim() {
    assert_eq!(will_i_win_nim(1), true);
    assert_eq!(will_i_win_nim(2), true);
    assert_eq!(will_i_win_nim(3), true);
    assert_eq!(will_i_win_nim(4), false);
    assert_eq!(will_i_win_nim(5), true);
    assert_eq!(will_i_win_nim(6), true);
    assert_eq!(will_i_win_nim(7), true);
    assert_eq!(will_i_win_nim(8), false);
    assert_eq!(will_i_win_nim(9), true);
    assert_eq!(will_i_win_nim(10), true);
    assert_eq!(will_i_win_nim(11), true);
    assert_eq!(will_i_win_nim(12), false);
}