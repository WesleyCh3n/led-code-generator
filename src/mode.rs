use std::os::raw::c_uchar;

#[derive(Clone, Copy, Debug)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Rainbow,
    Blink,
}

impl Mode {
    pub fn get_buf(m: Mode, len: usize) -> Vec<Vec<Color>> {
        match m {
            // Mode::Rainbow => rainbow_vec(len),
            Mode::Rainbow => rainbow_buf_call(len),
            Mode::Blink => blink_vec(len),
        }
    }
}

extern "C" {
    fn rainbow_buf(len: c_uchar, offset: c_uchar) -> *const c_uchar;
    fn deallocate_buf(buf: *const c_uchar);
}

fn rainbow_buf_call(len: usize) -> Vec<Vec<Color>> {
    let mut result = Vec::new();
    unsafe {
        for offset in 0..len {
            let ptr = rainbow_buf(len as u8, offset as u8);
            let data: Vec<Color> = std::slice::from_raw_parts(ptr, len * 3)
                .windows(3)
                .map(|c| Color(c[0], c[1], c[2]))
                .step_by(3)
                .collect();
            deallocate_buf(ptr);
            result.push(data);
        }
    }
    result
}

pub fn rainbow_vec(len: usize) -> Vec<Vec<Color>> {
    let mut buf = Vec::new();
    for i in 0..len {
        buf.push(rainbow(i as f32 / len as f32));
    }
    let mut result = Vec::new();
    for _ in 0..len {
        result.push(buf.clone());
        let first = buf.remove(0);
        buf.push(first);
    }
    result
}

pub fn rainbow(ratio: f32) -> Color {
    let region = (ratio * 6.0) as i32;

    let normalized: i32 = (ratio * 256.0 * 6.0).floor() as i32;
    let x = (normalized % 256) as u8;

    match region {
        0 => Color(255, x, 0),
        1 => Color(255 - x, 255, 0),
        2 => Color(0, 255, x),
        3 => Color(0, 255 - x, 255),
        4 => Color(x, 0, 255),
        5 => Color(255, 0, 255 - x),
        _ => Color(0, 0, 0),
    }
}

pub fn blink_vec(len: usize) -> Vec<Vec<Color>> {
    let mut result = Vec::new();
    for i in 0..12 {
        result.push(vec![Color(255 - i * 20, 0, 0); len]);
    }
    for i in (0..12).rev() {
        result.push(vec![Color(255 - i * 20, 0, 0); len]);
    }
    result
}
