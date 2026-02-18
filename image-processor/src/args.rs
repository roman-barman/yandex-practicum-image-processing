pub mod validation;

const SUPPORTED_IMAGE_FORMATS: &[&str] = &["png"];

///Image processor CLI
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input image file path
    #[clap(long, value_parser = parse_input_file_path)]
    pub input: std::path::PathBuf,

    /// Output image file path
    #[clap(long, value_parser = parse_output_file_path)]
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

fn parse_input_file_path(path: &str) -> Result<std::path::PathBuf, String> {
    let path = std::path::PathBuf::from(path);
    if !path.exists() {
        return Err(format!("input file '{}' does not exist", path.display()));
    }
    if !is_supported_image_format(&path) {
        return Err(format!(
            "input file '{}' is not a supported image format (supported formats: {:?})",
            path.display(),
            SUPPORTED_IMAGE_FORMATS
        ));
    }

    Ok(path)
}

fn parse_output_file_path(path: &str) -> Result<std::path::PathBuf, String> {
    let path = std::path::PathBuf::from(path);
    if !is_supported_image_format(&path) {
        return Err(format!(
            "output file '{}' is not a supported image format (supported formats: {:?})",
            path.display(),
            SUPPORTED_IMAGE_FORMATS
        ));
    }
    Ok(path)
}

fn is_supported_image_format(image_path: &std::path::Path) -> bool {
    SUPPORTED_IMAGE_FORMATS.contains(
        &image_path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or(""),
    )
}
