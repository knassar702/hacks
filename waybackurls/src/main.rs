use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    lines.for_each(|domain| {
        get_webarchive(str::trim(&domain.unwrap()));
    });
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct WebArchive {
    urls: Vec<String>,
}

fn get_webarchive(query: &str) {
    let url = format!(
        "http://web.archive.org/cdx/search/cdx?url=*.{}/*&output=json&fl=original&collapse=urlkey",
        query
    );
    match ureq::get(&url).call() {
        Ok(res) => {
            let json_body: Vec<WebArchive> = res.into_json().unwrap();
            json_body.iter().for_each(|item| {
                item.urls
                    .iter()
                    .filter(|url| !url.contains("original"))
                    .for_each(|y| {
                        println!("{}", y);
                    });
            });
        }
        Err(_err) => {
            // IGNORE
        }
    }
}
