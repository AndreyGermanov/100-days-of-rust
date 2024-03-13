use std::collections::HashMap;
fn l33t(input: &str) -> String {
    let hash:HashMap<char,&str> = HashMap::from([
        ('A',"4"),('B',"6"),('E',"3"),('I',"1"),('M',"(V)"),('N',"(\\)"),('O',"0"),('S',"5"),
        ('T',"7"),('V',"\\/"),('W',"`//")]);
    input.chars().fold("".to_string(),|accum,ch| {
        format!("{}{}",accum,hash.get(&ch).unwrap_or(&ch.to_string().as_str()))
    })
}

#[test]
fn test_l33t() {
    assert_eq!(l33t("BASIC"),"6451C");
    assert_eq!(l33t("ELEET"),"3L337");
    assert_eq!(l33t("WOW"),"`//0`//");
    assert_eq!(l33t("MOM"),"(V)0(V)");
}
