use std::path::PathBuf;

use clap::Parser;
use rinafy::{errors::RinafyResult, mask::MaskKind, rinafy_file};

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

    #[arg(short, long, default_value_t = MaskKind::NoMask)]
    mask: MaskKind,
}

fn main() -> RinafyResult<()> {
    let args = Args::parse();

    let mask = args.mask.create_mask()?;

    rinafy_file(&args.in_file, &args.out_file, &mask)
}
