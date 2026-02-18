pub(crate) fn validate_args(
    args: &crate::args::Args,
    default_plugin_directory: &std::path::Path,
    lib_extension: &str,
) -> Result<(), anyhow::Error> {
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
