use std::fs::File;
use std::error;
use std::io::Read;
use std::path::PathBuf;

use base64::read::DecoderReader;
use base64::Config;
use base64::CharacterSet;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Opt::from_args();
    let mut decoded = String::new();

    let mut file = File::open(args.path)?;

    let charset = match args.charset.to_lowercase().as_str() {
        "standard" => CharacterSet::Standard,
        "urlsafe" => CharacterSet::UrlSafe,
        "crypt" => CharacterSet::Crypt,
        "bcrypt" => CharacterSet::Bcrypt,
        "imapmutf7" => CharacterSet::ImapMutf7,
        "binhex" => CharacterSet::BinHex,
        _ => Err("Invalid character set")?,
    };
    let mut file = DecoderReader::new(
        &mut file, 
        Config::new(charset, true)
    );
    file.read_to_string(&mut decoded)?; // TODO: Doesn't like newlines
    println!("{decoded}");

    Ok(())
}

#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "b64d", about = "Decode a base64 encoded file.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    #[structopt(short, long, default_value = "standard")]
    charset: String
}