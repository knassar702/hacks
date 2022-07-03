use std::io::{self, BufRead};

mod app;
mod cli;

fn main() {
    let threads = cli::get_args().threads;
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let kxss = app::App {
        input: lines.map(|x| x.unwrap()).collect(),
    };
    kxss.start(threads);
}
