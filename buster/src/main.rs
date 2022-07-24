
use std::{time::Instant, fs, io::Error};
use reqwest;
use rayon::prelude::*;
use clap::Parser;

/// simple web server fuzzing
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// url to fuzz
    #[clap(value_parser)]
    url: String,

    /// input file
    #[clap(value_parser)]
    file: String,

    /// exclude status code
    #[clap(short, long, value_parser, default_value_t = 404)]
    exclude_status_code: u16,

    /// number of threads to use
    #[clap(short, long, value_parser, default_value_t = 8)]
    threads: u8,
}


fn main() {
    let now = Instant::now();

    let args = Args::parse();

    let input = match read_file(args.file) {
        Err(err) => {println!("{:#?}", err); return;},
        Ok(i) => i
    };
    
    let client = reqwest::blocking::Client::new();
    
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads.into()).build().unwrap();
    
    println!("probing: {}", args.url);
    
    pool.install(|| {
        input.par_iter().for_each(|op| {
            
            let uri = args.url.clone() + op;
            
            let resp = client.get(&uri).send();
            
            let resp = match resp {
                Err(err) => { println!("{:#?}", err); return;},
                Ok(r) => r
            };
            
            if !(resp.status() == args.exclude_status_code) {
                println!("uri: {}, status: {:#?}", uri, resp.status());
            }
        });
    });

    println!("tested {} items", input.len());
    let elapsed_time = now.elapsed();
    println!("Running took {} ms", elapsed_time.as_millis());
}


fn read_file(file: String) -> Result<Vec<String>, Error> {
    let contents = match fs::read_to_string(file) {
        Err(err) => return Err(err),
        Ok(c) => c
    };

    let input: Vec<String> = contents.split("\n")
        .filter(|line| !line.starts_with("#"))
        .map(|line| String::from(line))
        .collect();

    return Ok(input);
}