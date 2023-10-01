use std::io::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use signal_hook;

pub struct EndSignal {
  pub end_manual: Arc<AtomicBool>,
  pub end_systemd: Arc<AtomicBool>,
}

impl EndSignal {
  pub fn new() -> Result<EndSignal, Error> {

    let out = EndSignal {
      end_manual: Arc::new(AtomicBool::new(false)),
      end_systemd: Arc::new(AtomicBool::new(false)),
    };

    // manual end signal
    signal_hook::flag::register(signal_hook::consts::SIGINT, std::sync::Arc::clone(&out.end_manual))?;

    // systemd end signal
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&out.end_systemd))?;

    return Ok(out);
  }

  pub fn is_end(&self) -> bool {
    return self.end_manual.load(Ordering::Relaxed) || self.end_systemd.load(Ordering::Relaxed);
  }
}
