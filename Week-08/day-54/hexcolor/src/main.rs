use std::str::FromStr;
use regex::{Regex};

fn rgb_to_hex(rgb: &str) -> Result<String,&str> {
    let s: String = rgb.to_string().chars().filter(|ch| !ch.is_whitespace()).collect();
    let re = Regex::new(r"rgb\(?([0-9]{0,4}),([0-9]{0,4}),([0-9]{0,4})\)").unwrap();
    if let Some(caps) = re.captures(s.as_str()) {
        Ok(caps.iter().skip(1).fold("#".to_string(), |accum, cap| {
            format!("{}{:0>2x}",accum, u8::from_str(cap.unwrap().as_str()).unwrap())
        }))
    } else {
        Err("Incorrect input")
    }
}

#[test]
fn test_rgb_to_hex() {
    assert_eq!(rgb_to_hex("rgb(0, 128, 192)").unwrap(), "#0080c0");
    assert_eq!(rgb_to_hex("rgb(45, 255, 192)").unwrap(), "#2dffc0");
    assert_eq!(rgb_to_hex("rgb(0, 0, 0)").unwrap(),"#000000");
    assert!(rgb_to_hex("").is_err());
    assert!(rgb_to_hex("fsdf").is_err());
    assert!(rgb_to_hex("rgb(fd, fsdf, fsdf)").is_err());
}