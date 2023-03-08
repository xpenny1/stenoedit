use std::io::stdout;

use crossterm::{terminal::{self, Clear, ClearType}, ExecutableCommand, cursor::MoveTo};

pub (crate) struct RawMode;
impl RawMode {
    pub (crate) fn enable() -> RawMode {
        terminal::enable_raw_mode().expect("Fehler: RawMode");
        stdout().execute(Clear(terminal::ClearType::All)).unwrap();
        RawMode
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        stdout().execute(Clear(ClearType::All)).unwrap();
        stdout().execute(MoveTo(0, 0)).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
