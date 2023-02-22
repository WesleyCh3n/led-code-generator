use std::sync::{Arc, Mutex};

use crate::mode::{Color, Mode, ModeConfig};

pub struct Strip {
    pub buf: Arc<Mutex<Vec<Color>>>, // in write, out read
    pub mode_cfg: Arc<Mutex<ModeConfig>>, // in read, out write
    pub radius: f32,                 // ui config
}

impl Strip {
    pub fn start(
        buf: Arc<Mutex<Vec<Color>>>,
        mode_cfg: Arc<Mutex<ModeConfig>>,
    ) {
        let mut prev_mode_cfg = *mode_cfg.lock().unwrap();
        let mut cycle = Mode::get_buf(prev_mode_cfg).into_iter().cycle();
        std::thread::spawn(move || loop {
            // if mode or len change, recreate buf
            let curr_mode_cfg = *mode_cfg.lock().unwrap();
            if curr_mode_cfg != prev_mode_cfg {
                cycle = Mode::get_buf(curr_mode_cfg).into_iter().cycle();
                prev_mode_cfg = curr_mode_cfg;
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
            *buf.lock().unwrap() = cycle.next().unwrap();
        });
    }
}

impl Default for Strip {
    fn default() -> Self {
        let buf = Arc::new(Mutex::new(Vec::new()));
        let mode_cfg = Arc::new(Mutex::new(ModeConfig::default()));

        Self::start(buf.clone(), mode_cfg.clone());
        Self {
            buf,
            mode_cfg,
            radius: 20.0,
        }
    }
}
