use std::io::{self, BufRead,BufReader};
use scraper::{Html, Selector};
use rayon::prelude::*;
use ureq::Response;
use std::{
    env,
    fs::File,
    path::Path
};

fn requester(url: &str) -> Result<ureq:: Response, ureq::Error> {
    ureq::get(url).call() 
}

fn extract_title(res: Response,url: &str,selector: &Selector){
    let body = Html::parse_document(res.into_string().expect("failed to parse document").as_str());

    let title = body.select(selector).next()
        .unwrap()
        .text()
        .collect::<Vec<_>>();

    println!("{} ({})",&title[0],url.replace("\n",""));
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("failed to read file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let selector = Selector::parse("title").unwrap();
    let mut urls = Vec::new();

    if args.len() >= 2 {
        urls = lines_from_file(&args[1]);
    }else { 
        loop{
            let mut input = String::from("");
            io::stdin().read_line(&mut input).expect("failed to read from pipe");
            if input != "" {
                urls.push(input);
            }else{
                break
            }
        }
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(40)
        .build()
        .unwrap();
    pool.install(|| {
        urls.par_iter().for_each(|url| {
            match requester(url.as_str()){
                Ok(resp) => extract_title(resp, url.as_str(), &selector),
                Err(e) => eprintln!("{}",e)}
            }
        )}
    )
}
