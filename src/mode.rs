use std::os::raw::c_uchar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Rainbow,
    Blink,
}

impl Mode {
    pub fn get_buf(cfg: ModeConfig) -> Vec<Vec<Color>> {
        match cfg.mode {
            Mode::Rainbow => rainbow_buf_call(cfg.len),
            Mode::Blink => breath_buf_call(cfg.len),
        }
    }
    pub fn get_code(&self) -> String {
        let lines: Vec<String> = include_str!("../c/algorithm.c")
            .lines()
            .into_iter()
            .map(|s| s.into())
            .collect();
        match *self {
            Mode::Rainbow => lines[8..65].join("\n"),
            Mode::Blink => lines[65..].join("\n"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ModeConfig {
    pub mode: Mode,
    pub len: usize,
    pub speed: usize,
    color: Option<Color>,
}

impl Default for ModeConfig {
    fn default() -> Self {
        Self {
            mode: Mode::Rainbow,
            len: 15,
            speed: 100,
            color: None,
        }
    }
}

extern "C" {
    fn rainbow_buf(len: c_uchar, offset: c_uchar) -> *const c_uchar;
    fn breath_buf(
        len: c_uchar,
        period: c_uchar,
        offset: c_uchar,
        c1: *const c_uchar,
        c2: *const c_uchar,
    ) -> *const c_uchar;
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

fn breath_buf_call(len: usize) -> Vec<Vec<Color>> {
    let mut result = Vec::new();
    unsafe {
        let period = 6;
        for offset in 0..period * 2 {
            let c1: [c_uchar; 3] = [255, 0, 0];
            let c2: [c_uchar; 3] = [0, 255, 0];
            let ptr =
                breath_buf(len as u8, period, offset, c1.as_ptr(), c2.as_ptr());
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
