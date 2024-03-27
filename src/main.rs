use std::path::PathBuf;

use clap::Parser;
use rinafy::rinafy_file;

// A program to "Rinafy" any image, which is just to shift its colors along a gradient and create an animation this way.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // The image file you want to Rinafy.
    #[arg(short, long)]
    in_file: PathBuf,

    // The place that the Rinafy-ed image shall be put.
    #[arg(short, long)]
    out_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    if let Err(_error) = rinafy_file(args.in_file, args.out_file) {
        eprintln!("Something went wrong, exiting!");
    }
}
