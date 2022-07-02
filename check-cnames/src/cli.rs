use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "check-cnames",
    about = "Resolves CNAMEs and checks if they are valid"
)]
pub struct Opt {
    #[structopt(short = "t", long = "thread", default_value = "30")]
    pub threads: u32,
}

pub fn get_args() -> Opt {
    Opt::from_args()
}
