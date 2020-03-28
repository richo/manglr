use std::env;
use std::io::{
    self,
    BufRead,
};

use rand::{
    self,
    Rng,
};

fn apply_meme<'a, F: FnMut(char) -> Option<String>>(input: &'a str, mut transform: F) -> String {
    let mut out = String::with_capacity(input.len());
    for c in input.chars() {
        if c.is_alphabetic() {
            if let Some(s) = transform(c) {
                out.push_str(&s)
            } else {
                out.push(c)
            }
        } else {
            out.push(c)
        }
    }
    out
}

fn spongebob<'a>(input: &'a str) -> String {
    let mut rng = rand::thread_rng();
    apply_meme(input, move |c| {
        if rng.gen() {
            Some(c.to_uppercase().collect())
        } else {
            None
        }
    })
}


fn embiggen<'a>(input: &'a str) -> String {
    apply_meme(&input, |c| { Some(format!(":big-{}:", c)) })
}

fn main() {
    let args = env::args();
    let inner: Vec<_> = args.map(|x| &x[..]).collect();
    let f = match &inner[..] {
        &["spongebob"] => {
            spongebob
        },
        &["embiggen"] => {
            embiggen
        },
        _ => {
            println!("Usage: $0 spongebob|embiggen");
            return
        },
    };

    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            println!("{}", f(&line));
        }
    }
}
