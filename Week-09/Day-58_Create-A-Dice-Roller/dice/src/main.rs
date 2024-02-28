use std::io;
use std::io::BufRead;
use std::str::FromStr;
use rand::Rng;
use regex::Regex;

fn main() {
    for line in io::stdin().lock().lines() {
        match line {
            Ok(line) => {
                let dices = get_dices(line.as_str());
                if dices.len() == 0  { continue }
                dices.iter().for_each(|dice| println!("{}", dice));
                break;
            },
            _ => continue
        }
    }
}

fn get_dices(input: &str) -> Vec<String> {
    input.trim()
        .split(" ")
        .map(|item| get_dice(item))
        .filter(|item| item.len()>0)
        .collect()
}

fn get_dice(input: &str) -> String {
    let re = Regex::new("[0-9]*").unwrap();
    let nums:Vec<_> = input.split("d")
        .filter(|item| {
            match re.find(item) {
                Some(result) => result.start() == 0 && result.end() == item.len(),
                None => false
            }
        })
        .map(|item| { u64::from_str(item)})
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap())
        .filter(|item| *item > 0)
        .collect();
    if nums.len() != 2 { return "".to_string()};
    let result = (0..nums[0])
        .map(|_| rand::thread_rng().gen_range(1..nums[1]+1))
        .fold((0,vec![]),|mut accum,item| {
            accum.0 += item;
            accum.1.push(item.to_string());
            accum
        });
    format!("{}: {}", result.0, result.1.join(" "))
}
