pub mod validation;
#[cfg(target_os = "linux")]
const LIB_EXTENSION: &str = "so";
#[cfg(target_os = "windows")]
const LIB_EXTENSION: &str = "dll";
const DEFAULT_PLUGIN_DIRECTORY: &str = "target/debug";

const SUPPORTED_IMAGE_FORMATS: &[&str] = &["png"];

///Image processor CLI
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input image file path
    #[clap(long, value_parser = parse_input_file_path)]
    input: std::path::PathBuf,

    /// Output image file path
    #[clap(long, value_parser = parse_output_file_path)]
    output: std::path::PathBuf,

    /// Plugin name
    #[clap(long, value_parser = parse_plugin_name)]
    plugin: String,

    /// Plugin parameters file path
    #[clap(long, value_parser = parse_params_file_path)]
    params: std::path::PathBuf,

    /// Plugin directory path
    #[clap(long, value_parser = parse_plugin_directory)]
    plugin_path: Option<std::path::PathBuf>,
}

impl Args {
    pub fn plugin_path(&self) -> std::path::PathBuf {
        let plugin_name = if self.plugin.ends_with(LIB_EXTENSION) {
            self.plugin.clone()
        } else {
            format!("{}.{}", self.plugin, LIB_EXTENSION)
        };

        let plugin_directory = self
            .plugin_path
            .as_deref()
            .unwrap_or(std::path::Path::new(DEFAULT_PLUGIN_DIRECTORY));
        plugin_directory.join(plugin_name)
    }

    pub fn get_input(&self) -> &std::path::Path {
        &self.input
    }

    pub fn get_output(&self) -> &std::path::Path {
        &self.output
    }

    pub fn get_params(&self) -> &std::path::Path {
        &self.params
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

fn parse_plugin_name(path: &str) -> Result<String, String> {
    if path.is_empty() {
        return Err("plugin name is empty".to_string());
    }
    Ok(path.to_string())
}

fn parse_params_file_path(path: &str) -> Result<std::path::PathBuf, String> {
    let path = std::path::PathBuf::from(path);
    if !path.exists() {
        return Err(format!(
            "plugin parameters file '{}' does not exist",
            path.display()
        ));
    }
    Ok(path)
}

fn parse_plugin_directory(path: &str) -> Result<std::path::PathBuf, String> {
    if path.is_empty() {
        return Err("plugin directory is empty".to_string());
    }
    let path = std::path::PathBuf::from(path);
    if !path.exists() {
        return Err(format!(
            "plugin directory '{}' does not exist",
            path.display()
        ));
    }
    if !path.is_dir() {
        return Err(format!(
            "plugin directory '{}' is not a directory",
            path.display()
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
