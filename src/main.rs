use crossterm::cursor::{MoveTo, MoveRight, MoveLeft, MoveDown};
use crossterm::terminal::{Clear, ClearType, size};
use crossterm::{terminal, ExecutableCommand};
use std::io::{self, stdout};
use std::io::Read;

struct RawMode;
impl RawMode {
    pub fn enable() -> RawMode {
        terminal::enable_raw_mode().expect("Fehler: RawMode");
        stdout().execute(Clear(ClearType::All)).unwrap();
        RawMode
    }
}
impl Drop for RawMode {
    fn drop(&mut self) {
        stdout().execute(Clear(ClearType::All)).unwrap();
        stdout().execute(MoveTo(0,0)).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

fn draw_frame() {
    let (size_x,size_y) = size().unwrap();
    for i in 0..size_y {
       stdout().execute(MoveTo(0,i)).unwrap();
       print!("~\r");
       stdout().execute(MoveTo(size_x,i)).unwrap();
       print!("~\r");
    }
    for i in 0..size_x {
       stdout().execute(MoveTo(i,0)).unwrap();
       print!("~\r");
       stdout().execute(MoveTo(i,size_y)).unwrap();
       print!("~\r");
    }
    stdout().execute(MoveTo(1,1)).unwrap();
}

fn main() {
    let _raw_mode = RawMode::enable();
    draw_frame();
    let mut buffer = [0;1];
    while io::stdin().read(&mut buffer).unwrap() == 1 && buffer != [b'q'] {
        let input_char = buffer[0] as char;
        if input_char == '\n' || input_char == '\r' {
            print!("{}", input_char);
           stdout().execute(MoveDown(1)).unwrap()
               .execute(MoveRight(1)).unwrap();
        } else {
            print!("{}", input_char);
            stdout().execute(MoveRight(1)).unwrap();
            stdout().execute(MoveLeft(1)).unwrap();
        }
        
    }
}
















