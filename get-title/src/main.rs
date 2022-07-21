use std::io;
use scraper::{Html, Selector};
use rayon::prelude::*;
use ureq::Response;
use structopt::StructOpt;

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

/// Takes a list of urls and prints their titles
#[derive(StructOpt, Debug)]
#[structopt(name = "get-title")]
struct Opt {
    /// max concurrent
    #[structopt(short,default_value="40")]
    c: usize,
}

fn main() {
    let opt = Opt::from_args();
    let selector = Selector::parse("title").unwrap();
    let mut urls = Vec::new();

    loop{
        let mut input = String::from("");
        io::stdin().read_line(&mut input).expect("failed to read from pipe");
        if input != "" {
            urls.push(input);
        }else{
            break
        }
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(opt.c)
        .build()
        .unwrap();
    pool.install(|| {
        urls.par_iter().for_each(|url| {
            match requester(url.as_str()){
                Ok(resp) => extract_title(resp, url.as_str().trim_whitespace(), &selector),
                Err(e) => eprintln!("{}",e)}
            }
        )}
    )
}
