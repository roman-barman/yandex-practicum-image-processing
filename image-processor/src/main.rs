use clap::Parser;
use std::ffi::CString;
mod args;
mod plugin_loader;

#[cfg(target_os = "linux")]
const LIB_EXTENSION: &str = "so";
#[cfg(target_os = "windows")]
const LIB_EXTENSION: &str = "dll";
const DEFAULT_PLUGIN_DIRECTORY: &str = "target/debug";
const SUPPORTED_IMAGE_FORMATS: &[&str] = &["png"];

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();
    let default_plugin_directory = std::path::Path::new(DEFAULT_PLUGIN_DIRECTORY);
    args::validation::validate_args(
        &args,
        default_plugin_directory,
        LIB_EXTENSION,
        SUPPORTED_IMAGE_FORMATS,
    )?;

    let plugin_path = args.plugin_path(default_plugin_directory, LIB_EXTENSION);
    let plugin = plugin_loader::Plugin::new(
        plugin_path
            .to_str()
            .ok_or(anyhow::anyhow!("plugin path is not valid"))?,
    )
    .map_err(|_| anyhow::anyhow!("failed to load plugin"))?;
    let plugin = plugin
        .interface()
        .map_err(|_| anyhow::anyhow!("failed to load plugin interface"))?;

    let mut image = image::open(&args.input)
        .map_err(|_| anyhow::anyhow!("failed to open input image"))?
        .to_rgba8();
    let params = std::fs::read_to_string(&args.params)
        .map_err(|_| anyhow::anyhow!("failed to read plugin parameters"))?;
    let params = CString::new(params)
        .map_err(|_| anyhow::anyhow!("failed to convert plugin parameters to CString"))?;

    (plugin.process_image)(
        image.width(),
        image.height(),
        image.as_mut_ptr(),
        params.as_ptr(),
    );

    image
        .save(&args.output)
        .map_err(|_| anyhow::anyhow!("failed to save output image"))?;

    Ok(())
}
