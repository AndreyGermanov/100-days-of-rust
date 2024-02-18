fn tree(num:usize) -> Vec<String> {
    (0..num).fold(vec![],|mut accum,n| {
        let pad: String = vec![' '; num-n-1].iter().collect();
        let symb: String = vec!['#'; 2*n+1].iter().collect();
        accum.push(pad.to_owned() + symb.as_str() + pad.as_str()); accum
    })
}

#[test]
fn test_tree() {
    assert_eq!(tree(1), vec!["#"]);
    assert_eq!(tree(2), vec![" # ",
                                  "###"
    ]);
    assert_eq!(tree(5), vec!["    #    ",
                                  "   ###   ",
                                  "  #####  ",
                                  " ####### ",
                                  "#########"
    ]);
    assert_eq!(tree(0), vec![] as Vec<String>);
}
