use std::sync::{Arc, Mutex};

use crate::mode::{Color, Mode};

pub struct Strip {
    pub mode: Arc<Mutex<Mode>>,
    pub len: Arc<Mutex<usize>>,
    pub buf: Arc<Mutex<Vec<Color>>>,
    // pub buf: Arc<Mutex<Vec<Color>>>,
    // pub rx: std::sync::mpsc::Receiver<Vec<Color>>,
    pub speed: Arc<Mutex<usize>>,
    pub radius: f32, // ui config
}

impl Strip {
    pub fn start(
        mode: Arc<Mutex<Mode>>,
        len: Arc<Mutex<usize>>,
        buf: Arc<Mutex<Vec<Color>>>,
        speed: Arc<Mutex<usize>>,
    ) {
        let mut prev_mode = *mode.lock().unwrap();
        let mut prev_len = *len.lock().unwrap();
        let mut cycle = Mode::get_buf(prev_mode, prev_len).into_iter().cycle();
        std::thread::spawn(move || loop {
            // if mode or len change, recreate buf
            let curr_mode = *mode.lock().unwrap();
            let curr_len = *len.lock().unwrap();
            if curr_mode != prev_mode || curr_len != prev_len {
                cycle = Mode::get_buf(curr_mode, curr_len).into_iter().cycle();
                prev_mode = curr_mode;
                prev_len = curr_len;
            }

            let timeout = *speed.lock().unwrap() as u64;
            std::thread::sleep(std::time::Duration::from_millis(timeout));
            *buf.lock().unwrap() = cycle.next().unwrap();
        });
    }
}

impl Default for Strip {
    fn default() -> Self {
        let len = Arc::new(Mutex::new(20));
        let mode = Arc::new(Mutex::new(Mode::Rainbow));
        let buf = Arc::new(Mutex::new(Vec::new()));
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
