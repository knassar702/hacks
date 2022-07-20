use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "httprobe",
    about = "Take a list of domains and probe for working HTTP and HTTPS servers "
)]
pub struct Opt {
    #[structopt(short = "w", long = "workers", default_value = "30")]
    pub threads: usize,
    #[structopt(long = "http", default_value = "80,8080")]
    pub http_ports: String,
    #[structopt(long = "https", default_value = "443,8443")]
    pub https_ports: String,
    #[structopt(short = "t", long = "timeout", default_value = "10")]
    pub timeout: usize,
    #[structopt(short = "m", long = "mode", default_value = "normal", possible_values = &["normal","large", "xlarge"])]
    pub mode: String,
}

pub fn get_args() -> Opt {
    Opt::from_args()
}
