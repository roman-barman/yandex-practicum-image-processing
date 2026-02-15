pub(crate) fn validate_args(
    args: &crate::args::Args,
    default_plugin_directory: &std::path::Path,
    lib_extension: &str,
    supported_image_formats: &[&str],
) -> Result<(), anyhow::Error> {
    if !args.input.exists() {
        return Err(anyhow::anyhow!(
            "input file '{}' does not exist",
            args.input.display()
        ));
    }
    if !is_supported_image_format(supported_image_formats, &args.input) {
        return Err(anyhow::anyhow!(
            "input file '{}' is not a supported image format (supported formats: {:?})",
            args.input.display(),
            supported_image_formats
        ));
    }
    if !is_supported_image_format(supported_image_formats, &args.output) {
        return Err(anyhow::anyhow!(
            "output file '{}' is not a supported image format (supported formats: {:?})",
            args.output.display(),
            supported_image_formats
        ));
    }
    if args.plugin.is_empty() {
        return Err(anyhow::anyhow!("plugin name is empty"));
    }
    if !args.params.exists() {
        return Err(anyhow::anyhow!(
            "plugin parameters file '{}' does not exist",
            args.params.display()
        ));
    }
    if let Some(plugin_directory) = &args.plugin_path {
        if !plugin_directory.exists() {
            return Err(anyhow::anyhow!(
                "plugin directory '{}' does not exist",
                plugin_directory.display()
            ));
        }
        if !plugin_directory.is_dir() {
            return Err(anyhow::anyhow!(
                "plugin directory '{}' is not a directory.",
                plugin_directory.display()
            ));
        }
    } else {
        if !default_plugin_directory.exists() {
            return Err(anyhow::anyhow!(
                "default plugin directory '{}' does not exist",
                default_plugin_directory.display()
            ));
        }
        if !default_plugin_directory.is_dir() {
            return Err(anyhow::anyhow!(
                "default plugin directory '{}' is not a directory.",
                default_plugin_directory.display()
            ));
        }
    };
    let plugin_path = args.plugin_path(default_plugin_directory, lib_extension);
    if !plugin_path.exists() {
        return Err(anyhow::anyhow!(
            "plugin '{}' does not exist",
            plugin_path.display()
        ));
    }

    Ok(())
}

fn is_supported_image_format(
    supported_image_formats: &[&str],
    image_path: &std::path::Path,
) -> bool {
    supported_image_formats.contains(
        &image_path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or(""),
    )
}
