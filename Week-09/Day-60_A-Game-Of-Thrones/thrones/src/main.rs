use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    for line in io::stdin().lock().lines() {
        match line {
            Ok(line) if u32::from_str(line.as_str()).is_ok() => {
                play(u32::from_str(line.as_str()).unwrap());
                break;
            },
            _ => continue
        }
    }
}

fn play(mut num: u32) {
    while num > 1 {
        let op:i32 = if (num + 1) % 3 == 0 {1} else if (num - 1) % 3 == 0 {-1} else {0};
        println!("{} {}", num, op);
        num = (num as i32 + op) as u32 / 3;
    }
    println!("{}", num);
}

