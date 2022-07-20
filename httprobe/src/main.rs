use std::io::{self, BufRead};
mod app;
mod cli;

fn main() {
    let stdin_input = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let cli_args = cli::get_args();
    let https_ports = cli_args
        .https_ports
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let http_ports = cli_args
        .http_ports
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let mode: app::Mode;

    match cli_args.mode.as_str() {
        "xlarge" => mode = app::Mode::XLarge,
        "large" => mode = app::Mode::Large,
        _ => mode = app::Mode::Normal,
    }

    let httprobe = app::Httprobe {
        urls: stdin_input,
        threads: cli_args.threads,
        timeout: cli_args.timeout,
        https_ports,
        http_ports,
        mode,
    };
    httprobe.start();
}
