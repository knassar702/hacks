use std::io::{self, BufRead};

mod app;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let anti_burl = app::App {
        input: lines.collect::<Result<Vec<_>, _>>().unwrap(),
    };
    anti_burl.start()
}
