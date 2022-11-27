use dns_lookup::lookup_host;
use structopt::StructOpt;
use rayon::prelude::*;
use std::io;

/// Takes domains on stdin and output them on stdout if they resolve
#[derive(StructOpt,Debug)]
#[structopt(name="filter-resolved")]
struct Opt{ 
    /// concurrency level
    #[structopt(short,default_value="20")]
    c: usize
}

fn main(){
    let opt = Opt::from_args(); 
    let mut domains = Vec::new();

    loop {
        let mut input = String::from("");
        io::stdin().read_line(&mut input).expect("failed to read pipe");
        match input.as_str(){
            "" => break,
            "\n" => {},
            _ => domains.push(input.replace("\n","")),
        }
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(opt.c)
        .build()
        .unwrap();
    pool.install(|| {
        domains.par_iter().for_each(|domain|{
            match lookup_host(domain) {
                Ok(..) => {
                    println!("{}",domain);
                },
                Err(..) => {
                    // IGNORE
                },
            }
        })
    })
}
