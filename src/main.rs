use crossterm::cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, DisableBlinking, EnableBlinking, MoveUp};
use crossterm::event::poll;
use crossterm::terminal::{size, Clear, ClearType, EnableLineWrap};
use crossterm::{terminal, ExecutableCommand};
use device_query::{DeviceQuery, DeviceState, Keycode};
use serde_json::Value;
use std::collections::HashMap;
//use std::env;
use std::fs::read_to_string;
use std::io::Read;
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


fn main() {
//    let data = load_map();
//    generate_map();
//    let data = load_data();
//    let _raw_mode = RawMode::enable();
//    draw_frame();
//    let mut buffer = [0; 1];
//    while io::stdin().read(&mut buffer).unwrap() == 1 && buffer != [b'q'] && buffer != [b's'] {
//        let input_char = buffer[0] as char;
//        if input_char == '\n' || input_char == '\r' {
//            print!("{}", input_char);
//            stdout()
//                .execute(MoveDown(1))
//                .unwrap()
//                .execute(MoveRight(1))
//                .unwrap();
//        } else {
//            print!("{}", input_char);
//            stdout().execute(MoveRight(1)).unwrap();
//            stdout().execute(MoveLeft(1)).unwrap();
//        }
//    }
//    if buffer != [b'q'] {
//        let mut keys: Vec<Keycode> = Vec::new();
//        while keys != vec![Keycode::Q] {
//            // stdout().execute(Clear(ClearType::All)).unwrap();
//            //            draw_frame();
//            let keys2 = DeviceState::new().get_keys();
//            if keys2 != keys {
//                stdout().execute(Clear(ClearType::All)).unwrap();
//                draw_frame();
//                for key in keys2 {
//                    println!("{}\r", key);
//                    stdout().execute(MoveRight(1)).unwrap();
//                }
//            }
//            keys = DeviceState::new().get_keys();
//        }
//    }
    let _raw_mode: RawMode = RawMode::enable();
    let init_state: BufferState = BufferState { text: "".to_owned(), position: (1,1) };
    let mut buffer_state = init_state;
    let mut command: Command = Command::SwichMode(Mode::Normal);
    while command != Command::Exit {
        match command {
            Command::SwichMode(Mode::Normal) => {let (c,s) = normal_mode(buffer_state); buffer_state = s; command = c;}
            Command::SwichMode(Mode::Insert) => {let (c,s) = insert_mode(buffer_state); buffer_state = s; command = c;}
            _ => {command = Command::Exit;}
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
    stdout().execute(DisableBlinking).unwrap();
    stdout().execute(MoveTo(bstate.position.0,bstate.position.1)).unwrap();
    loop {
        if poll(Duration::from_millis(500)).unwrap() {
            let event: crossterm::event::Event = crossterm::event::read().unwrap();
            if is_key_press(&event, 'q') {
               return (Command::Exit, bstate); 
            }
            if let Some(c) = get_char(&event) {
               match c {
                  'h' => {stdout().execute(MoveLeft(1)).unwrap();},
                  'j' => {stdout().execute(MoveDown(1)).unwrap();},
                  'k' => {stdout().execute(MoveUp(1)).unwrap();},
                  'l' => {stdout().execute(MoveRight(1)).unwrap();},
                  'i' => return (Command::SwichMode(Mode::Insert), bstate.update_position()),
                  _ => {}
               }
            }
        }
    }
} 

fn insert_mode(mut bstate: BufferState) -> (Command, BufferState) {
    draw_frame();
    stdout().execute(MoveTo(bstate.position.0,bstate.position.1)).unwrap();
    stdout().execute(EnableBlinking).unwrap();
    loop {
        if poll(Duration::from_millis(500)).unwrap() {
            let event: crossterm::event::Event = crossterm::event::read().unwrap();
            if is_key_press(&event, 'q') {
               return (Command::Exit, bstate); 
            }
            if let Some(c) = get_char(&event) {
               print!("{}",c);
               bstate = bstate.update_position();
               refresh();
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
