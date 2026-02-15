use clap::Parser;
mod args;
mod plugin_loader;

#[cfg(target_os = "linux")]
const LIB_EXTENSION: &str = "so";
#[cfg(target_os = "windows")]
const LIB_EXTENSION: &str = "dll";
const DEFAULT_PLUGIN_DIRECTORY: &str = "target/debug";

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();
    let default_plugin_directory = std::path::Path::new(DEFAULT_PLUGIN_DIRECTORY);
    args::validation::validate_args(&args, default_plugin_directory, LIB_EXTENSION)?;

    Ok(())
}
