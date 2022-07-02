use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "anti-burl", about = "Takes URLs on stdin, prints them to stdout if they return a 200 OK. That's literally it")]
pub struct Opt {
    #[structopt(short = "t", long = "thread", default_value = "30")]
    pub threads: u32,
}

pub fn get_args() -> Opt{
    Opt::from_args()
}
