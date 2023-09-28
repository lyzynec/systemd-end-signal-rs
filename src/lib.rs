use std::io::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use signal_hook;

pub struct EndSignal {
  pub end: Arc<AtomicBool>,
}

impl EndSignal {
  pub fn new() -> Result<EndSignal, Error> {

    let out = EndSignal { end: Arc::new(AtomicBool::new(false)), };

    // manual end signal
    signal_hook::flag::register(signal_hook::consts::SIGINT, std::sync::Arc::clone(&out.end))?;

    // systemd end signal
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&out.end))?;

    return Ok(out);
  }

  pub fn is_end(&self) -> bool {
    return self.end.load(Ordering::Relaxed);
  }
}
