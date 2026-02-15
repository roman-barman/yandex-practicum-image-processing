use std::ffi::c_char;

#[unsafe(no_mangle)]
extern "C" fn process_image(width: u32, height: u32, rgb_data: *mut u8, params: *const c_char) {
    if width == 0 || height == 0 || params.is_null() || rgb_data.is_null() {
        return;
    }
    let image =
        unsafe { core::slice::from_raw_parts_mut(rgb_data, width as usize * height as usize * 4) };
    image.reverse();
}
