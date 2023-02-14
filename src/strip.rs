use std::sync::{Arc, Mutex};

use crate::mode::{Color, Mode};

pub struct Strip {
    pub mode: Arc<Mutex<Mode>>,
    pub len: Arc<Mutex<usize>>,
    pub buf: Arc<Mutex<Vec<Color>>>,
    pub speed: Arc<Mutex<usize>>,
    pub radius: f32, // ui config
}

impl Strip {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode: Arc::new(Mutex::new(mode)),
            len: Arc::new(Mutex::new(0)),
            radius: 30.0,
            speed: Arc::new(Mutex::new(100)),
            buf: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn start(
        mode: Arc<Mutex<Mode>>,
        len: Arc<Mutex<usize>>,
        buf: Arc<Mutex<Vec<Color>>>,
        speed: Arc<Mutex<usize>>,
    ) {
        let mut prev_mode = *mode.lock().unwrap();
        let mut prev_len = *len.lock().unwrap();
        std::thread::spawn(move || loop {
            // if mode or len change, recreate buf
            let curr_mode = *mode.lock().unwrap();
            let curr_len = *len.lock().unwrap();
            if curr_mode != prev_mode || curr_len != prev_len {
                *buf.lock().unwrap() = Mode::get_buf(curr_mode, curr_len);
                prev_mode = curr_mode;
                prev_len = curr_len;
            }

            let timeout = *speed.lock().unwrap() as u64;
            std::thread::sleep(std::time::Duration::from_millis(timeout));
            let buf = &mut *buf.lock().unwrap();
            if !buf.is_empty() {
                let first = buf.remove(0);
                buf.push(first);
            }
        });
    }
}

impl Default for Strip {
    fn default() -> Self {
        let len = Arc::new(Mutex::new(20));
        let mode = Arc::new(Mutex::new(Mode::Rainbow));
        let buf = Arc::new(Mutex::new(Mode::get_buf(Mode::Rainbow, 20)));
        let speed = Arc::new(Mutex::new(100));

        Self::start(mode.clone(), len.clone(), buf.clone(), speed.clone());
        Self {
            mode,
            len,
            buf,
            speed,
            radius: 20.0,
        }
    }
}
