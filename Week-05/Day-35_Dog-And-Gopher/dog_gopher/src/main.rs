mod field_map;

use std::io;
use std::path;
use std::fs::{File};
use field_map::read_fields_file;

fn gopher_escape_plan(filename: &str) -> Result<Vec<String>,io::Error> {
    match read_fields_file(File::open(path::Path::new(filename))?) {
        Ok(maps) => Ok(maps.iter().map(|item| { item.get_result() }).collect()),
        _ => Ok(vec!["BAD FILE!".to_string()])
    }
}

#[test]
fn test_gopher_escape_plan() {
    assert_eq!(gopher_escape_plan("day-35_sample_1_valid.txt").unwrap(),
        ["The gopher cannot escape.", "The gopher can escape through the hole at (2.500000,2.500000)."]);
    assert_eq!(gopher_escape_plan("day-35_sample_2_valid.txt").unwrap(),
        ["The gopher cannot escape.", "The gopher can escape through the hole at (2.500000,2.500000)."]);
    assert_eq!(gopher_escape_plan("day-35_sample_3_invalid.txt").unwrap(),
        ["BAD FILE!"]);
    assert_eq!(gopher_escape_plan("day-35_sample_4_invalid.txt").unwrap(),
        ["BAD FILE!"]);
    assert_eq!(gopher_escape_plan("day-35_sample_5_invalid.txt").unwrap(),
        ["BAD FILE!"]);
}