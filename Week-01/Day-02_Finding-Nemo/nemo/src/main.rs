use std::env;

fn main() {
    let word = "Nemo";

    let string = env::args()
        .skip(1)
        .reduce(|accum,item| accum+" "+&item)
        .expect("String argument required!");

    let index = string
        .split(" ")
        .position(|item| item == word)
        .expect(&format!("Could not find {}!",word));

    println!("I found Nemo at {}!", index);
}
