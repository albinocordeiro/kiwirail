use color_eyre::eyre::Result;
use kiwirail::kiwi_dot_converter::*;
use clap::Clap;

#[derive(Clap)]
#[clap(author = "Albino Cordeiro <albino@intuitionlogic.com", about = "Convert kiwi edges file to dot graph file format for visualization.")]
struct Options {
    #[clap(short, long, required = true, about = "Path to the input kiwi file")]
    file_path: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let options = Options::parse();
    kiwi2dot(&options.file_path)?;

    Ok(())
}
