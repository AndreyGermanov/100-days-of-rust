use serde_json::{Value};
use std::fs;
fn get_municipalities_count(state:&str) -> usize {
    serde_json::from_str::<Value>(fs::read_to_string("municipios.json").unwrap().as_str())
    .unwrap().as_array().unwrap().iter()
    .filter(|it| it["microrregiao"]["mesorregiao"]["UF"]["sigla"].as_str().unwrap()==state)
    .count()
}

#[test]
fn test_get_municipalities_count() {
    assert_eq!(get_municipalities_count("SP"),645)
}
