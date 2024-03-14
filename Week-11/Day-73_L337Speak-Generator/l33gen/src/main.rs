use std::collections::HashMap;

fn l33gen(input: &str) -> String {
    let hash:HashMap<&str,char> = HashMap::from([
        ("4",'A'),("6",'B'),("3",'E'),("1",'I'),("(V)",'M'),("(\\)",'N'),("0",'O'),("5",'S'),
        ("7",'T'),("\\/",'V'),("`//",'W')]);
    hash.iter().fold(input.to_string(), |result,(key,value)|
        result.replace(key,value.to_string().as_str())
    )
}

#[test]
fn test_l33gen() {
    assert_eq!(l33gen("6451C"),"BASIC");
    assert_eq!(l33gen("3L337"),"ELEET");
    assert_eq!(l33gen("`//0`//"),"WOW");
    assert_eq!(l33gen("(V)0(V)"),"MOM");
}
