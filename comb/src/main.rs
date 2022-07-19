use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
    mem,
    path::PathBuf,
};

use clap::Parser;

/// Combine the lines from two files in every combination
#[derive(Debug, Parser)]
#[clap(about, arg_required_else_help(true))]
struct Args {
    /// Flip mode (order by suffix)
    #[clap(short, long)]
    flip: bool,

    /// String to place between prefix and suffix
    #[clap(short, long, value_parser, default_value_t)]
    separator: String,

    /// Path to a text file with each line representing a prefix
    #[clap(value_parser)]
    prefixfile: PathBuf,

    /// Same as prefix file, but for suffixes
    #[clap(value_parser)]
    suffixfile: PathBuf,
}

fn main() -> io::Result<()> {
    let Args {
        flip,
        separator,
        prefixfile,
        suffixfile,
    } = Args::parse();
    let (mut outer, mut inner) = (prefixfile, suffixfile);
    if flip {
        mem::swap(&mut outer, &mut inner);
    }
    let outer = BufReader::new(File::open(outer)?);
    let mut inner = BufReader::new(File::open(inner)?);
    for outer in outer.lines() {
        let outer = outer?;
        inner.seek(io::SeekFrom::Start(0))?;
        for inner in (&mut inner).lines() {
            let inner = inner?;
            let (mut prefix, mut suffix) = (&outer, &inner);
            if flip {
                mem::swap(&mut prefix, &mut suffix);
            }
            println!("{prefix}{separator}{suffix}");
        }
    }
    Ok(())
}
