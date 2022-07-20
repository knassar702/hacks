use rayon::prelude::*;

const XLARGE: &str = include_str!("txt/xlarge.txt");
const LARGE: &str = include_str!("txt/large.txt");

pub enum Mode {
    Large,
    XLarge,
    Normal,
}



pub struct Httprobe {
    pub urls: Vec<String>,
    pub threads: usize,
    pub timeout: usize,
    pub https_ports: Vec<String>,
    pub http_ports: Vec<String>,
    pub mode: Mode,
}

impl Httprobe {
    pub fn start(&self) {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.threads.try_into().unwrap())
            .build()
            .unwrap();

        let urls = self.get_urls();
        pool.install(|| {
            urls.par_iter().for_each(|url| match self.send_req(url) {
                Ok(resp_url) => println!("{}", resp_url),
                Err(_) => {}
            });

        })
    }

    fn get_urls(&self) -> Vec<String> {
        let mut urls: Vec<String> = Vec::new();
        self.urls.iter().for_each(|url| {
            match self.mode {
                Mode::XLarge => {
                    let mut lines = XLARGE.lines();
                    while let Some(port) = lines.next() {
                        urls.push(format!("https://{}:{}", url, port));
                        urls.push(format!("http://{}:{}", url, port));
                    }
                }
                Mode::Large => {
                    let mut lines = LARGE.lines();
                    while let Some(port) = lines.next() {
                        urls.push(format!("https://{}:{}", url, port));
                        urls.push(format!("http://{}:{}", url, port));
                    }
                }
                _ => {}
            }
            self.https_ports.iter().for_each(|port| {
                urls.push(format!("https://{}:{}", url, port));
            });
            self.http_ports.iter().for_each(|port| {
                urls.push(format!("http://{}:{}", url, port));
            });

        });
        urls
    }

    fn send_req(&self, url: &str) -> Result<String, ureq::Error> {
        match ureq::builder()
            .redirects(0)
            .build()
            .get(url)
            .timeout(std::time::Duration::from_secs(
                self.timeout.try_into().unwrap(),
            ))
            .set("Connection", "close")
            .call()
        {
            Ok(res) => Ok(res.get_url().to_string()),
            Err(err) => Err(err),
        }
    }
}
