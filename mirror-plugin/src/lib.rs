use std::ffi::{CStr, c_char};

const PIXEL_SIZE: usize = 4;

#[unsafe(no_mangle)]
unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgb_data: *mut u8,
    params: *const c_char,
) {
    if width == 0 || height == 0 || params.is_null() || rgb_data.is_null() {
        return;
    }
    let len = match (width as usize)
        .checked_mul(height as usize)
        .and_then(|len| len.checked_mul(PIXEL_SIZE))
    {
        Some(len) => len,
        None => return,
    };
    let image = unsafe { core::slice::from_raw_parts_mut(rgb_data, len) };
    let params = unsafe { CStr::from_ptr(params) };
    let params = params.to_str().unwrap_or("");
    match params {
        "horizontal" => mirror_horizontal_rgba(image, width as usize, height as usize),
        "vertical" => mirror_vertical_rgba(image, width as usize, height as usize),
        _ => (),
    }
}

fn mirror_horizontal_rgba(buf: &mut [u8], width: usize, height: usize) {
    let stride = match width.checked_mul(PIXEL_SIZE) {
        Some(stride) => stride,
        None => return,
    };
    let mut out = vec![0u8; buf.len()];
    out.copy_from_slice(buf);

    for y in 0..height {
        let from = match y.checked_mul(stride) {
            Some(from) => from,
            None => return,
        };
        let to = match y.checked_add(1).and_then(|y| y.checked_mul(stride)) {
            Some(to) => to,
            None => return,
        };
        let row = &mut out[from..to];
        for x in 0..width / 2 {
            let l = match x.checked_mul(PIXEL_SIZE) {
                Some(l) => l,
                None => return,
            };
            let r = match width
                .checked_sub(1)
                .and_then(|r| r.checked_sub(x).and_then(|res| res.checked_mul(PIXEL_SIZE)))
            {
                Some(r) => r,
                None => return,
            };
            if l.checked_add(3).is_none() || r.checked_add(3).is_none() {
                return;
            }

            row.swap(l, r);
            row.swap(l + 1, r + 1);
            row.swap(l + 2, r + 2);
            row.swap(l + 3, r + 3);
        }
    }

    buf.copy_from_slice(&out);
}

fn mirror_vertical_rgba(buf: &mut [u8], width: usize, height: usize) {
    let stride = match width.checked_mul(PIXEL_SIZE) {
        Some(stride) => stride,
        None => return,
    };
    let mut out = vec![0u8; buf.len()];
    out.copy_from_slice(buf);

    for y in 0..height / 2 {
        let top = match y.checked_mul(stride) {
            Some(top) => top,
            None => return,
        };
        let bot = match height
            .checked_sub(1)
            .and_then(|l| l.checked_sub(y).and_then(|res| res.checked_mul(stride)))
        {
            Some(bot) => bot,
            None => return,
        };
        for i in 0..stride {
            if top.checked_add(i).is_none() || bot.checked_add(i).is_none() {
                return;
            }

            out.swap(top + i, bot + i);
        }
    }

    buf.copy_from_slice(&out);
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
