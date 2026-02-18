pub(crate) fn validate_args(args: &crate::args::Args) -> Result<(), anyhow::Error> {
    let plugin_path = args.plugin_path();
    if !plugin_path.exists() {
        return Err(anyhow::anyhow!(
            "plugin '{}' does not exist",
            plugin_path.display()
        ));
    }

    Ok(())
}
