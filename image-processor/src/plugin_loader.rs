use libloading::{Library, Symbol};
use std::ffi::c_char;

pub struct PluginInterface<'a> {
    pub process_image: Symbol<
        'a,
        extern "C" fn(width: u32, height: u32, rgb_data: *mut u8, params: *const c_char),
    >,
}

pub struct Plugin {
    plugin: Library,
}

impl Plugin {
    pub fn new(file_path: &str) -> Result<Self, libloading::Error> {
        Ok(Plugin {
            plugin: unsafe { Library::new(file_path) }?,
        })
    }
    pub fn interface(&self) -> Result<PluginInterface<'_>, libloading::Error> {
        Ok(PluginInterface {
            process_image: unsafe { self.plugin.get("process_image") }?,
        })
    }
}
