use std::io::stdout;

use crossterm::{terminal::size, cursor::MoveTo, ExecutableCommand};

fn draw_frame() {
    let (size_x, size_y) = size().unwrap();
    for i in 0..size_y {
        stdout().execute(MoveTo(0, i)).unwrap();
        print!("~\r");
        stdout().execute(MoveTo(size_x, i)).unwrap();
        print!("~\r");
    }
    for i in 0..size_x {
        stdout().execute(MoveTo(i, 0)).unwrap();
        print!("~\r");
        stdout().execute(MoveTo(i, size_y)).unwrap();
        print!("~\r");
    }
    stdout().execute(MoveTo(1, 1)).unwrap();
}
