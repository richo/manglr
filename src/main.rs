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
    let args: Vec<_> = env::args().collect();
    let this = &args[0];
    let inner: Vec<_> = args.iter().map(|x| x.as_str()).collect();
    let f = match &inner[..] {
        &[_, "spongebob"] => {
            spongebob
        },
        &[_, "embiggen"] => {
            embiggen
        },
        _ => {
            println!("Usage: {} <meme>", this);
            println!("Included memes:");
            println!("  spongebob");
            println!("  embiggen");
            return
        },
    };

    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            println!("{}", f(&line));
        }
    }
}
