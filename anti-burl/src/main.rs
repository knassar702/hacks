use std::io::{self, BufRead};

mod app;
mod cli;

fn main() {
    cli::get_args();
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let anti_burl = app::App {
        input: lines.collect::<Result<Vec<_>, _>>().unwrap(),
    };
    anti_burl.start(cli::get_args().threads);
}
