#[derive(Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Rainbow,
    Blink,
}

impl Mode {
    pub fn get_buf(m: Mode, len: usize) -> Vec<Color> {
        match m {
            Mode::Rainbow => rainbow_vec(len),
            Mode::Blink => blink_vec(len),
        }
    }
}

pub fn rainbow_vec(len: usize) -> Vec<Color> {
    let mut buf = Vec::new();
    for i in 0..len {
        buf.push(rainbow(i as f32 / len as f32));
    }
    buf
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

pub fn blink_vec(len: usize) -> Vec<Color> {
    let mut buf = Vec::new();
    for i in 0..len {
        buf.push(Color(255, 0, 0));
    }
    buf
}