fn check_lights(lights: &str) -> &str {
    let lights_vec = lights.chars().filter(|ch| *ch != ' ').clone().collect::<Vec<_>>();
    if lights_vec.len() > 15 || lights_vec.len() < 4 { return "ERROR"; }
    if !lights.starts_with("R") { return "REJECT"; }
    let mut prev_ch = ' ';
    lights_vec.iter().enumerate().fold("ACCEPT", |mut accum,(index,ch)| {
        if !"RYGPCX".contains(*ch) { accum = "ERROR"; }
        if *ch == 'X' && index != lights_vec.len() { return "ERROR"; }
        if *ch == 'G' && prev_ch != 'R' { return "REJECT"; }
        if *ch == 'Y' && prev_ch != 'G' { return "REJECT"; }
        if (*ch == 'C' || *ch == 'P') && (prev_ch != 'R') {  return "REJECT"; }
        prev_ch = *ch;
        return accum;
    })
}

#[test]
fn test_check_lights() {
    assert_eq!(check_lights("R G Y R C R G Y R"), "ACCEPT");
    assert_eq!(check_lights("G Y R G Y R"), "REJECT");
    assert_eq!(check_lights("R Y G P"), "REJECT");
    assert_eq!(check_lights("R G Y"), "ERROR");
    assert_eq!(check_lights("X 8 S"), "ERROR");
    assert_eq!(check_lights("R G Y R C R P R G Y R G Y R G Y R"), "ERROR");
}
