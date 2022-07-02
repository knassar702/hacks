mod req;

pub struct App {
    pub input: Vec<String>,
}

impl App {
    pub fn start(self) {
        let threader = rayon::ThreadPoolBuilder::new()
            .num_threads(50)
            .build()
            .unwrap();
        threader.install(|| {
            for line in self.input {
                let url = line.trim();
                match req::send_request(url) {
                    Ok(res) => {
                        if res.status() <= 300 || res.status() >= 500 {
                            println!(
                                "{} | {} | {}",
                                res.status(),
                                res.into_string().unwrap().len(),
                                url
                            );
                        }
                    }
                    Err(_err) => {
                        todo!();
                    }
                }
            }
        });
    }
}
