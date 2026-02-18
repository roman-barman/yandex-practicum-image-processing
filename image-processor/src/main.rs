use clap::Parser;
use std::ffi::CString;
mod args;
mod plugin_loader;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();
    args::validation::validate_args(&args)?;

    let plugin_path = args.plugin_path();
    let plugin = plugin_loader::Plugin::new(
        plugin_path
            .to_str()
            .ok_or(anyhow::anyhow!("plugin path is not valid"))?,
    )
    .map_err(|_| anyhow::anyhow!("failed to load plugin"))?;
    let plugin = plugin
        .interface()
        .map_err(|_| anyhow::anyhow!("failed to load plugin interface"))?;

    let mut image = image::open(args.get_input())
        .map_err(|_| anyhow::anyhow!("failed to open input image"))?
        .to_rgba8();
    let params = std::fs::read_to_string(args.get_params())
        .map_err(|_| anyhow::anyhow!("failed to read plugin parameters"))?;
    let params = CString::new(params)
        .map_err(|_| anyhow::anyhow!("failed to convert plugin parameters to CString"))?;

    // Safety: plugin is expected to honor the C ABI and not read/write outside the image buffer.
    unsafe {
        (plugin.process_image)(
            image.width(),
            image.height(),
            image.as_mut_ptr(),
            params.as_ptr(),
        )
    };

    image
        .save(args.get_output())
        .map_err(|_| anyhow::anyhow!("failed to save output image"))?;

    Ok(())
}
