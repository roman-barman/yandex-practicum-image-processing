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

impl Args {
    pub fn plugin_path(
        &self,
        default_plugin_directory: &std::path::Path,
        lib_extension: &str,
    ) -> std::path::PathBuf {
        let plugin_name = if self.plugin.ends_with(lib_extension) {
            self.plugin.clone()
        } else {
            format!("{}.{}", self.plugin, lib_extension)
        };

        let plugin_directory = self
            .plugin_path
            .as_deref()
            .unwrap_or(default_plugin_directory);
        plugin_directory.join(plugin_name)
    }
}
