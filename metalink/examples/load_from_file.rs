use anyhow::Result;
use clap::Parser;

/// Simple example for loading a metalink file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CLI {
    /// The path to the metalink file
    #[arg(short, long)]
    file_path: String,
}

fn main() -> Result<()> {
    let args = CLI::parse();

    let metalink = metalink::Metalink::load_from_file(args.file_path)?;
    println!("{:#?}", metalink);

    Ok(())
}
