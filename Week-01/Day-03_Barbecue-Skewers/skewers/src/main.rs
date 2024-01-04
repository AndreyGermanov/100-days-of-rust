fn get_skewer_types(skewers: &Vec<&str>) -> (i8, i8) {
    let mut veg_skewers = 0;
    let mut not_veg_skewers = 0;
    for skewer in skewers {
        if skewer.contains("x") {
            not_veg_skewers += 1;
        } else if skewer.contains("o") {
            veg_skewers += 1;
        }
    }
    return (veg_skewers, not_veg_skewers)
}

#[test]
fn test_get_skewer_types() {
    assert_eq!(get_skewer_types(&vec![
        "--xo--x--ox--",
        "--xx--x--xx--",
        "--oo--o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--"
    ]), (1, 4));
    assert_eq!(get_skewer_types(&vec![
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--"
    ]), (2, 3));
    assert_eq!(get_skewer_types(&vec![
        "--oooo-ooo--",
        "--xxxxxxxx--",
        "--o---",
        "-o-----o---x--",
        "--o---o-----"
    ]), (3, 2))
}
