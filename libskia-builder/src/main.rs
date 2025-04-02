use libskia_builder::latest_libskia;
use shared_library_builder::{build, LibraryTarget};
use std::error::Error;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Clone, Debug)]
struct BuildOptions {
    #[clap(long)]
    target_dir: Option<PathBuf>,
    /// Must be a path to a folder that contains `libskia`
    #[clap(long)]
    source_dir: Option<PathBuf>,
    #[clap(long, ignore_case = true)]
    target: Option<LibraryTarget>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let options: BuildOptions = BuildOptions::parse();

    let target_dir = options
        .target_dir
        .unwrap_or_else(|| PathBuf::from("target"));

    let src_dir = options.source_dir.unwrap_or_else(|| target_dir.join("src"));

    build(target_dir, src_dir, options.target, |target| {
        Ok(Box::new(latest_libskia(target)))
    })
}
