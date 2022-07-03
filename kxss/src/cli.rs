use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "kxss", about = "Get reflected XSS Parameters")]
pub struct Opt {
    #[structopt(short = "t", long = "thread", default_value = "30")]
    pub threads: u32,
}

pub fn get_args() -> Opt {
    Opt::from_args()
}
