use clap::{Args, Parser};
use clap_cargo::style::CLAP_STYLING;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(styles = CLAP_STYLING)]
pub enum CargoCli {
    Mkdocs(Opts),
}

#[derive(Args)]
pub struct Opts {
    #[arg(short = 'p', long)]
    pub document_private_items: bool,
    #[arg(short, long, default_value = "Cargo.toml")]
    pub manifest_file: PathBuf,
    //#[arg(short, long)]
    //pub feature: Vec<String>,
    //#[arg(long)]
    //pub all_features: bool,
    //#[arg(long)]
    //pub optional: bool,
    #[arg(short = 'x', long)]
    pub exclude: Vec<String>,
    #[arg(short = 'n', long)]
    pub include: Vec<String>,
    #[arg(short, long)]
    pub dev_dependencies: bool,
    #[arg(short, long)]
    pub build_dependencies: bool,
    #[arg(short, long)]
    pub open: bool,
}

impl Default for Opts {
    fn default() -> Self {
        match CargoCli::parse() {
            CargoCli::Mkdocs(opts) => opts,
        }
    }
}
