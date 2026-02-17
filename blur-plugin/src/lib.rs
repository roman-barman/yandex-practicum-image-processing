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
    let (radius, iterations) = parse_params(params);
    if radius == 0 || iterations == 0 {
        return;
    }
    blur_rgba(image, width as usize, height as usize, radius, iterations);
}

fn parse_params(params: &str) -> (usize, usize) {
    let trimmed = params.trim();
    if trimmed.is_empty() {
        return (0, 0);
    }
    let params = trimmed.split(';').collect::<Vec<_>>();
    let radius = match params.iter().find(|param| param.starts_with("radius=")) {
        Some(value) => value
            .strip_prefix("radius=")
            .unwrap_or("0")
            .parse()
            .unwrap_or(0),
        None => 0,
    };
    let iterations = match params.iter().find(|param| param.starts_with("iterations=")) {
        Some(value) => value
            .strip_prefix("iterations=")
            .unwrap_or("0")
            .parse()
            .unwrap_or(0),
        None => 0,
    };
    (radius, iterations)
}

fn blur_rgba(buf: &mut [u8], width: usize, height: usize, radius: usize, iterations: usize) {
    let max_radius = width.saturating_sub(1).max(height.saturating_sub(1));
    let radius = radius.min(max_radius);
    if radius == 0 {
        return;
    }
    let stride = match width.checked_mul(PIXEL_SIZE) {
        Some(stride) => stride,
        None => return,
    };

    for _ in 0..iterations {
        let mut out = vec![0u8; buf.len()];
        for y in 0..height {
            let y0 = y.saturating_sub(radius);
            let y1 = match y.checked_add(radius) {
                Some(y1) => y1.min(height - 1),
                None => break,
            };
            for x in 0..width {
                let x0 = x.saturating_sub(radius);
                let x1 = match x.checked_add(radius) {
                    Some(x1) => x1.min(width - 1),
                    None => break,
                };

                let mut sum = [0u32; 4];
                let mut count = 0u32;
                for yy in y0..=y1 {
                    let row = match yy.checked_mul(stride) {
                        Some(row) => row,
                        None => return,
                    };
                    for xx in x0..=x1 {
                        let idx = match xx.checked_mul(PIXEL_SIZE) {
                            Some(idx) => match row.checked_add(idx) {
                                Some(idx) => idx,
                                None => return,
                            },
                            None => return,
                        };
                        sum[0] += buf[idx] as u32;
                        sum[1] += buf[idx + 1] as u32;
                        sum[2] += buf[idx + 2] as u32;
                        sum[3] += buf[idx + 3] as u32;
                        count += 1;
                    }
                }

                let out_idx = match y.checked_mul(stride) {
                    Some(right) => match x.checked_mul(PIXEL_SIZE) {
                        Some(left) => match right.checked_add(left) {
                            Some(idx) => idx,
                            None => return,
                        },
                        None => return,
                    },
                    None => return,
                };
                out[out_idx] = (sum[0] / count) as u8;
                out[out_idx + 1] = (sum[1] / count) as u8;
                out[out_idx + 2] = (sum[2] / count) as u8;
                out[out_idx + 3] = (sum[3] / count) as u8;
            }
        }
        buf.copy_from_slice(&out);
    }
}

#[cfg(test)]
mod test {
    use crate::{blur_rgba, parse_params};

    #[test]
    fn parse_params_test() {
        assert_eq!(parse_params(""), (0, 0));
        assert_eq!(parse_params("radius=1"), (1, 0));
        assert_eq!(parse_params("radius=1;iterations=2"), (1, 2));
        assert_eq!(parse_params("iterations=2"), (0, 2));
    }

    #[test]
    fn blur_test() {
        let mut buf = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
        ];
        blur_rgba(&mut buf, 3, 3, 2, 2);
        assert_eq!(
            buf,
            [
                17, 18, 19, 20, 17, 18, 19, 20, 17, 18, 19, 20, 17, 18, 19, 20, 17, 18, 19, 20, 17,
                18, 19, 20, 17, 18, 19, 20, 17, 18, 19, 20, 17, 18, 19, 20
            ]
        );
    }
}
