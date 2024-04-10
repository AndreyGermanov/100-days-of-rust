use image::GenericImageView;

fn main() {
    let img = image::open("./rtx3090.webp").unwrap();
    println!("{}",POINTS.iter().enumerate().map(|(idx, (x,y))| {
        let symbol = (img.get_pixel(*x,*y).0[0] as char).to_string();
        if idx == 0 { symbol.to_uppercase() } else { symbol }
    }).collect::<Vec<_>>().join(""));
}

const POINTS: [(u32, u32); 11] = [(821, 246),(704, 271),(861, 275),(838, 268),(753, 257),
    (713, 279),(703, 267),(584, 280),(718, 267),(845, 272),(857, 258)];
