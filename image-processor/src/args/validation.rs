pub(crate) fn validate_args(
    args: &crate::args::Args,
    default_plugin_directory: &std::path::Path,
    lib_extension: &str,
) -> Result<(), anyhow::Error> {
    if !args.input.exists() {
        return Err(anyhow::anyhow!(
            "input file '{}' does not exist",
            args.input.display()
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
    let plugin_directory = if let Some(plugin_directory) = &args.plugin_path {
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
        plugin_directory
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
        default_plugin_directory
    };
    if args.plugin.ends_with(lib_extension) {
        validate_plugin_exists(plugin_directory, &args.plugin)?;
    } else {
        validate_plugin_exists(
            plugin_directory,
            &format!("{}.{}", args.plugin, lib_extension),
        )?;
    }

    Ok(())
}

fn validate_plugin_exists(
    plugin_directory: &std::path::Path,
    plugin_name: &str,
) -> Result<(), anyhow::Error> {
    let path = plugin_directory.join(plugin_name);
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "plugin '{}' does not exist in directory '{}'",
            plugin_name,
            plugin_directory.display()
        ));
    }
    Ok(())
}
