use std::io::{self, BufRead};

mod app;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let cname = app::App{input: lines.map(|x| x.unwrap()).collect()};
    cname.start();
}
