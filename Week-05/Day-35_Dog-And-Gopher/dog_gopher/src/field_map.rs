use std::fs::File;
use std::io;
use std::io::Read;
use std::num::ParseFloatError;
use std::str::FromStr;

struct FieldMap {
    gopher_pos: (f32, f32),
    dog_pos: (f32, f32),
    holes: Vec<(f32, f32)>
}

impl FieldMap {
    fn new() -> Self {
        FieldMap{gopher_pos: (0.0,0.0), dog_pos:(0.0,0.0), holes:vec![]}
    }

    fn from_str(num: usize, content: &str) -> Option<Self> {
        let mut result = FieldMap::new();
        let lines:Vec<_> = content.split("\n").filter(|line| line.trim().len() > 0).collect();
        for index in 0..lines.len() {
            let values_opt = FieldMap::get_values(lines[index].split(" ")
                .map(|item| item.trim()).filter(|item| item.len()>0).collect());
            if values_opt.is_err() { return None }
            let values = values_opt.unwrap();
            if index == 0 {
                if values.len() != 5 { return None }
                if values[0] != num as f32 { return None }
                result.gopher_pos = (values[1], values[2]);
                result.dog_pos = (values[3], values[4])
            } else {
                if values.len() != 2 { return None }
                result.holes.push((values[0], values[1]))
            }
        }
        Some(result)
    }

    fn get_values(items: Vec<&str>) -> Result<Vec<f32>,ParseFloatError> {
        items.iter().map(|item| f32::from_str(item)).collect()
    }
}

pub trait EscapeResult {
    fn get_result(&self) -> String;
}

impl EscapeResult for FieldMap {
    fn get_result(&self) -> String {
        for hole in &self.holes {
            let gopher_time = f32::powi(f32::abs(hole.0 - self.gopher_pos.0),2) +
                f32::powi(f32::abs(hole.1 - self.gopher_pos.1),2);
            let dog_time = f32::powi(f32::abs(hole.0 - self.dog_pos.0),2) +
                f32::powi(f32::abs(hole.1 - self.dog_pos.1),2) / 2.0;
            if gopher_time < dog_time {
                return format!("The gopher can escape through the hole at ({:.6},{:.6}).",hole.0, hole.1)
            }
        }
        return "The gopher cannot escape.".to_string();
    }
}

pub fn read_fields_file(mut file: File) -> Result<Vec<Box<dyn EscapeResult>>, io::Error> {
    let mut result:Vec<Box<dyn EscapeResult>> = vec![];
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let lines:Vec<_> = text.split("\n\n").filter(|item| item.trim().len() > 0).collect();
    for index in 0..lines.len() {
        let field_map = FieldMap::from_str(index+1, lines[index]);
        if field_map.is_none() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput,"Incorrect file"))
        }
        result.push(Box::new(field_map.unwrap()));
    }
    return Ok(result);
}