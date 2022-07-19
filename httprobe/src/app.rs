use rayon::prelude::*;

const XLARGE: &str = include_str!("txt/xlarge.txt");
const LARGE: &str = include_str!("txt/large.txt");

pub enum Mode {
    Normal,
    Large,
    XLarge,
}

pub struct Httprobe {
    pub urls: Vec<String>,
    pub threads: usize,
    pub timeout: usize,
}

impl Httprobe {
    pub fn start(&self, mode: Mode) {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.threads.try_into().unwrap())
            .build()
            .unwrap();
        let mut urls: Vec<String> = Vec::new();
        self.urls.iter().for_each(|url| {
            match mode {
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
                Mode::Normal => {
                    // split the url into host and ports
                    let mut lines = self.urls.iter();
                    while let Some(line) = lines.next() {
                        let mut url = line.split(":");
                        let host = url.next().unwrap();
                        let port = url.next().unwrap();
                        urls.push(format!("https://{}:{}", host, port));
                        urls.push(format!("http://{}:{}", host, port));
                    }
                }
            }
        });

        pool.install(|| {
            urls.par_iter().for_each(|url| match self.send_req(url) {
                Ok(resp_url) => println!("{}", resp_url),
                Err(_) => {}
            });
        })
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
