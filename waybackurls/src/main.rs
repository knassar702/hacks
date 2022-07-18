use serde::{Deserialize, Serialize};

fn main() {
    get_webarchive("knas.me");
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
            let mut urls: Vec<String> = Vec::new();
            json_body.iter().for_each(|item| {
                item.urls
                    .iter()
                    .filter(|url| !url.contains("original"))
                    .for_each(|y| {
                        urls.push(y.to_string());
                    });
            });
            println!("{:?}", urls);
        }
        Err(_err) => {
            // IGNORE
        }
    }
}
