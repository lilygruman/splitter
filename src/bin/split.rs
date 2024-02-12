use std::{
    fs::File,
    io::{self, stdin},
};

use clap::Parser;
use clap_num::si_number;

#[derive(Parser)]
#[command(version, about = "Split a file into chunks of a given maximum size")]
struct Args {
    /// Path to file to be split
    source: Option<String>,

    /// Path to put the chunks in
    #[arg(short, long = "dest")]
    destination: String,

    /// Max chunk size, in bytes
    #[arg(short, long, value_parser=si_number::<u64>)]
    size: u64,
}

fn main() -> io::Result<()> {
    let Args {
        source,
        destination,
        size,
    } = Args::parse();
    if let Some(source) = source {
        splitter::split(&mut File::open(source)?, &destination, size)
    } else {
        splitter::split(&mut stdin(), &destination, size)
    }
}
