
pub fn insert_mode(mut bstate: BufferState) -> (Command, BufferState) {
    draw_frame();
    let mut stdout = stdout();
    stdout.execute(MoveTo(bstate.position.0,bstate.position.1)).unwrap();
    stdout.execute(EnableBlinking).unwrap().execute(cursor::SetCursorShape(CursorShape::Line)).unwrap();
    loop {
        if poll(Duration::from_millis(500)).unwrap() {
            let event: crossterm::event::Event = crossterm::event::read().unwrap();
//            if is_key_press(&event, 'q') {
//               return (Command::Exit, bstate); 
//            }
//               stdout.execute(style::Print(format!("{:?}",event))).unwrap();
            if let Some(c) = get_char(&event) {
//               print!("{}",c);
//               stdout.execute(MoveDown(1)).unwrap();
//               stdout.execute(MoveLeft(1)).unwrap();
//               stdout.execute(style::PrintStyledContent("█".dark_magenta())).unwrap();
//               stdout.execute(style::PrintStyledContent(c.white())).unwrap();
//               stdout.execute(style::Print(c)).unwrap();
//               stdout.execute(MoveUp(1)).unwrap();
//               bstate = bstate.update_position();
//               refresh();
                execute!(stdout,
                         Print(c),
                         MoveLeft(1),
                         MoveDown(1),
                         PrintStyledContent(c.dark_green()),
                         MoveUp(1),
                         MoveLeft(1),
                         MoveUp(1),
                         PrintStyledContent(c.dark_magenta()),
                         MoveDown(1),
                         ResetColor
                         );
            }
            if let crossterm::event::Event::Key(k) = event {
                if k.code == crossterm::event::KeyCode::Esc {
                   return (Command::SwichMode(Mode::Normal), bstate.update_position()); 
                }
            }
        }
    }
}