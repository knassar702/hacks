use injector::{Injector, Urlinjector};
use rayon::prelude::*;
use url::Url;
mod injector;

const REFLECTED_TEXT: &str = "iy3j4h234hjb23234";

pub struct App {
    pub input: Vec<String>,
}

impl App {
    pub fn start(&self, threads: u32) {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads.try_into().unwrap())
            .build()
            .unwrap();
        pool.install(|| {
            self.input.par_iter().for_each(|url| {
                let current_injector = Injector{
                    request: Url::parse(url).unwrap(),
                    keep_value: false
                };
                current_injector.url_value(REFLECTED_TEXT).iter().for_each(|url_param|{
                    url_param.1.iter().for_each(|new_url|{
                        match ureq::get(new_url.as_str()).set("User-agent","User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.100 Safari/537.36").call() {
                            Ok(res) => {
                                if res.content_type().contains("html") {
                                    res.into_string().unwrap().lines().for_each(|line|{
                                        if line.contains(REFLECTED_TEXT) {
                                            self.check_xss(url.as_str(),url_param.0);
                                        }
                                    });
                                }
                            },
                            Err(_) => {
                                // IGNORE
                            }
                        }
                    });
                });
            });
        });
    }

    fn check_xss(&self, url: &str, parameter: &str) {
        let xss_checks = vec!["\"", "'", "<", ">"];
        let mut found: Vec<String> = Vec::new();
        xss_checks.iter().for_each(|current_check| {
            let current_injector = Injector{
                request: Url::parse(url).unwrap(),
                keep_value: false
            };
            let payload = format!("kexs{}",current_check);
            let new_url = current_injector.set_urlvalue(parameter,payload.as_str());
            match ureq::get(new_url.as_str()).set("User-agent","User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.100 Safari/537.36").call() {
                Ok(res) => {
                    if res.content_type().contains("html") {
                        res.into_string().unwrap().lines().for_each(|line|{
                            if line.contains(payload.as_str()) {
                                found.push(payload.clone());
                            }
                        });
                    }
                },
                Err(_) => {
                    // IGNORE
                }
            }
        });
        if found.len() > 0 {
            println!(
                "param '{}' is reflected and allows {} on {}",
                parameter,
                found.join(" ,"),
                url
            );
        }
    }
}
