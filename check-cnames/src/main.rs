use std::io::{self, BufRead};

mod app;
mod cli;

fn main() {
    let threads = cli::get_args().threads;
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let cname = app::App {
        input: lines.map(|x| x.unwrap()).collect(),
    };
    cname.start(threads);
}
