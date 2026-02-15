pub mod validation;

///Image processor CLI
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input image file path
    #[clap(long)]
    pub input: std::path::PathBuf,

    /// Output image file path
    #[clap(long)]
    pub output: std::path::PathBuf,

    /// Plugin name
    #[clap(long)]
    pub plugin: String,

    /// Plugin parameters file path
    #[clap(long)]
    pub params: std::path::PathBuf,

    /// Plugin directory path
    #[clap(long)]
    pub plugin_path: Option<std::path::PathBuf>,
}
