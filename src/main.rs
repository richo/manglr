use std::io::{
    self,
    BufRead,
};
use rand;
use rand::Rng;

fn spongebob<'a>(input: &'a str) -> String {
    let mut rng = rand::thread_rng();
    input.chars().map(|c| {
        if c.is_alphabetic() && rng.gen() {
            c.to_uppercase().next().expect("Couldn't uppercase")
        } else {
            c
        }
    }).collect()
}

fn main() {
    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            println!("{}", spongebob(&line));
        }
    }
}
