use std::env;
use std::process::{Command, Stdio};
use std::io::{
    self,
    BufRead,
    Write,
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

trait Sink {
    fn sink<'a>(&self, line: &'a str);
}

struct StdoutSink;

impl Sink for StdoutSink {
    #[cfg(target_os = "macos")]
    fn sink<'a>(&self, line: &'a str) {
        println!("{}", line);
    }
}

struct ClipboardSink;

impl Sink for ClipboardSink {
    fn sink<'a>(&self, line: &'a str) {
        let child = Command::new("pbcopy")
            .stdin(Stdio::piped())
            .spawn()
            .expect("failed to execute process");
        child.stdin.expect("Couldn't get child stdin").write_all(line.as_bytes())
            .expect("couldn't write to child");
    }
}

fn get_sink() -> Box<dyn Sink> {
    match env::var("MANGLR_SINK").ok() {
        Some(ref s) if s == "clipboard" => {
            Box::new(ClipboardSink)
        },
        None => {
            Box::new(StdoutSink)
        },
        Some(other) => {
            panic!("Unknown sink: {}", other)
        },
    }
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

    let sink = get_sink();

    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            sink.sink(&f(&line));
        }
    }
}
