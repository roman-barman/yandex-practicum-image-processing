use std::ffi::{CStr, c_char};

const PIXEL_SIZE: usize = 4;

#[unsafe(no_mangle)]
extern "C" fn process_image(width: u32, height: u32, rgb_data: *mut u8, params: *const c_char) {
    if width == 0 || height == 0 || params.is_null() || rgb_data.is_null() {
        return;
    }
    let image = unsafe {
        core::slice::from_raw_parts_mut(rgb_data, width as usize * height as usize * PIXEL_SIZE)
    };
    let params = unsafe { CStr::from_ptr(params) };
    let params = params.to_str().unwrap_or("");
    match params {
        "horizontal" => mirror_horizontal_rgba(image, width as usize, height as usize),
        "vertical" => mirror_vertical_rgba(image, width as usize, height as usize),
        _ => (),
    }
}

fn mirror_horizontal_rgba(buf: &mut [u8], width: usize, height: usize) {
    let stride = width * PIXEL_SIZE;
    for y in 0..height {
        let row = &mut buf[y * stride..(y + 1) * stride];
        for x in 0..width / 2 {
            let l = x * PIXEL_SIZE;
            let r = (width - 1 - x) * PIXEL_SIZE;
            row.swap(l, r);
            row.swap(l + 1, r + 1);
            row.swap(l + 2, r + 2);
            row.swap(l + 3, r + 3);
        }
    }
}

fn mirror_vertical_rgba(buf: &mut [u8], width: usize, height: usize) {
    let stride = width * 4;
    for y in 0..height / 2 {
        let top = y * stride;
        let bot = (height - 1 - y) * stride;
        for i in 0..stride {
            buf.swap(top + i, bot + i);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{mirror_horizontal_rgba, mirror_vertical_rgba};

    #[test]
    fn horizontal_mirror() {
        let mut buf = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        mirror_horizontal_rgba(&mut buf, 2, 2);
        assert_eq!(buf, [5, 6, 7, 8, 1, 2, 3, 4, 13, 14, 15, 16, 9, 10, 11, 12]);
    }

    #[test]
    fn vertical_mirror() {
        let mut buf = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        mirror_vertical_rgba(&mut buf, 2, 2);
        assert_eq!(buf, [9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
