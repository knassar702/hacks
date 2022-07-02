use rayon::prelude::*;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

pub struct App {
    pub input: Vec<String>
}


impl App {
    pub fn start(self) {
        self.input.par_iter().for_each(|domain| {
            let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
            let response = resolver.lookup_ip(domain).unwrap();
            println!("{}", response.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
        });
    }
}
