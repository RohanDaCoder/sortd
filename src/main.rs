use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    folder: PathBuf,

    #[arg(short, long, default_value_t = false)]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();

    println!("🔍 Scanning folder: {}", args.folder.display());
}
