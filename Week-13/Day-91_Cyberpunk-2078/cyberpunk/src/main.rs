use std::collections::HashMap;
fn cyberpunk(month: &str, num: usize) -> String {
    let months:HashMap<&str,usize> = HashMap::from([("January",0),("February",1),("March",2),
        ("April",3),("May",4),("June",5),("July",6),("August",7),("Septempter",8),("Octorber",9),
        ("November",10),("December",11)]);
    let months_rev:HashMap<usize,&str> = HashMap::from_iter(
        months.iter().map(|(value,index)| (*index,*value))
    );
    months_rev[&((months[month]+num) % 12)].to_string()
}

#[test]
fn test_cyberpunk() {
    assert_eq!(cyberpunk("November",3),"February");
    assert_eq!(cyberpunk("May",24), "May");
}


