use crossterm::cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, DisableBlinking, EnableBlinking, MoveUp, self, CursorShape};
use crossterm::event::poll;
use crossterm::style::{Stylize, Print, PrintStyledContent, ResetColor};
use crossterm::terminal::{size, Clear, ClearType, EnableLineWrap};
use crossterm::{terminal, ExecutableCommand, style, execute};
use device_query::{DeviceQuery, DeviceState, Keycode};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
//use std::env;
use std::fs::read_to_string;
use std::io::{Read, stdin};
use std::io::{self, stdout};
use std::string::String;
use std::time::Duration;

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
        stdout().execute(MoveTo(0, 0)).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

#[derive(PartialEq)]
enum Mode {
    Normal,
    Insert,
    Steno,
}

#[derive(PartialEq)]
enum Command {
    SwichMode(Mode),
    Exit,
}

struct BufferState {
    text: String,
    position: (u16,u16)
}

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


//static DATA2: phf::Map<&'static str, &str> = phf_map! {
//    "hallo" => "bye",
//    "bye" => "hallo",
//    "123" => "456",
//};

include!("../data/codegens.rs");

fn main() {
//    let data = load_map();
//    generate_map();
//    let data = load_data();
//    for (k,v) in DATA.to_iter() {
//        println("key: {k}, value: {v}");
//    }
//    println!("{:?}", DATA);
//    let mut r: String = String::new();
//    loop {
//        stdin().read_line(&mut r).unwrap();
//        println!("{:?}", DATA.get(r.trim()));
//        r = "".to_owned();
//        r = "".to_owned();
//    }
    let _raw_mode: RawMode = RawMode::enable();
    let init_state: BufferState = BufferState { text: "".to_owned(), position: (1,1) };
    let mut buffer_state = init_state;
    let mut command: Command = Command::SwichMode(Mode::Normal);
    while command != Command::Exit {
        match command {
            Command::SwichMode(Mode::Normal) => {let (c,s) = normal_mode(buffer_state); buffer_state = s; command = c;}
            Command::SwichMode(Mode::Insert) => {let (c,s) = insert_mode(buffer_state); buffer_state = s; command = c;}
            Command::SwichMode(Mode::Steno) => {let (c,s) = steno_mode(buffer_state); buffer_state = s; command = c;}
            _ => {command = Command::Exit;}
        }
    }
}

fn steno_mode(mut buffer_state: BufferState) -> (Command, BufferState) {
//    let mut keys = Vec::new();
//    while keys != vec![Keycode::Escape] {
//    if poll(Duration::from_millis(500)).unwrap() {
//        if keys != DeviceState::new().get_keys() {
//            for key in &keys {
//                stdout().execute(style::Print(key));
//            }
//            keys = DeviceState::new().get_keys();
//        }
//    }
//    }
    while !DeviceState::new().get_keys().contains(&Keycode::S) {}
    loop {
    let mut keys: HashSet<Keycode> = HashSet::new();
    let mut new_keys = DeviceState::new().get_keys();
    while new_keys != vec![] {
        for key in &new_keys {
             keys.insert(*key); 
         }
        new_keys = DeviceState::new().get_keys(); 
    }
    if keys.contains(&Keycode::Escape){
       return (Command::SwichMode(Mode::Normal),buffer_state.update_position());
    } else {
        for key in keys {
            stdout().execute(style::Print(key)).unwrap();
        }
    }
    }

}

fn is_key_press(e: &crossterm::event::Event,key: char) -> bool {
    if let crossterm::event::Event::Key(k) = e {
        if let crossterm::event::KeyCode::Char(c) = k.code {
            if c == key {
                return true;
            } 
        } 
    }
    false
}

fn get_char(e: &crossterm::event::Event) -> Option<char> {
    if let crossterm::event::Event::Key(k) = e {
        if let crossterm::event::KeyCode::Char(c) = k.code {
            return Some(c);
        } 
    }
    None
}

fn refresh() {
    stdout().execute(MoveRight(1)).unwrap().execute(MoveLeft(1)).unwrap(); 
}

impl BufferState {
   fn update_position(mut self) -> BufferState {
       self.position = crossterm::cursor::position().unwrap();
       self
   }
}

fn normal_mode(mut bstate: BufferState) -> (Command, BufferState) {
    draw_frame();
    while poll(Duration::from_millis(500)).unwrap() {
        crossterm::event::read();
    }
    let mut stdout = stdout();
    stdout.execute(DisableBlinking).unwrap().execute(cursor::SetCursorShape(CursorShape::Block)).unwrap();
    stdout.execute(MoveTo(bstate.position.0,bstate.position.1)).unwrap();
    loop {
        if poll(Duration::from_millis(500)).unwrap() {
            let event: crossterm::event::Event = crossterm::event::read().unwrap();
            if is_key_press(&event, 'q') {
               return (Command::Exit, bstate); 
            }
            if let Some(c) = get_char(&event) {
               match c {
                  'h' => {stdout.execute(MoveLeft(1)).unwrap();},
                  'j' => {stdout.execute(MoveDown(1)).unwrap();},
                  'k' => {stdout.execute(MoveUp(1)).unwrap();},
                  'l' => {stdout.execute(MoveRight(1)).unwrap();},
                  '0' => {stdout.execute(MoveTo(1,crossterm::cursor::position().unwrap().1)).unwrap();},
                  '$' => {stdout.execute(MoveTo(crossterm::terminal::size().unwrap().0 - 2,crossterm::cursor::position().unwrap().1)).unwrap();},
                  'i' => return (Command::SwichMode(Mode::Insert), bstate.update_position()),
                  's' => return (Command::SwichMode(Mode::Steno), bstate.update_position()),
                  _ => {}
               }
            }
        }
    }
} 

fn insert_mode(mut bstate: BufferState) -> (Command, BufferState) {
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

fn load_data() -> HashMap<String, String> {
    serde_json::from_str::<HashMap<String, Value>>(&read_to_string("./data/palantype-DE.json").unwrap()) 
    .unwrap()
    .into_iter()
    .map(|(x, y): (String, Value)| {
        if let Value::String(s) = y {
            (x, s)
        } else {
            panic!("JSON");
        }
    })
    .collect()
}

//fn generate_map() {
//    env::set_var("OUT_DIR", "data");
//    let data: HashMap<String, String> = serde_json::from_str::<HashMap<String, Value>>(
//        include_str!("../data/palantype-DE.json"), /* "{\"x\":\"y\"}"*/
//    ) //        "{
//    .unwrap()
//    .into_iter()
//    .map(|(x, y): (String, Value)| {
//        if let Value::String(s) = y {
//            (x, s)
//        } else {
//            panic!("JSON");
//        }
//    })
//    .collect();
//    uneval::to_out_dir(data, "data.rs").unwrap();
//}
//
//fn load_map() -> HashMap<String, String> {
//    include!("../data/data.rs");
//}
