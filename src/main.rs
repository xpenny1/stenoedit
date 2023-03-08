//use crossterm::cursor::{
//    self, CursorShape, DisableBlinking, EnableBlinking, MoveDown, MoveLeft, MoveRight, MoveTo,
//    MoveUp,
//};
//use crossterm::event::poll;
//use crossterm::style::{Print, PrintStyledContent, ResetColor, Stylize};
//use crossterm::terminal::{size, Clear, ClearType, EnableLineWrap};
//use crossterm::{execute, style, terminal, ExecutableCommand};
//use device_query::{DeviceQuery, DeviceState, Keycode};
//use serde_json::Value;
//use std::collections::{HashMap, HashSet};
////use std::env;
//use std::fs::read_to_string;
//use std::io::{self, stdout};
//use std::io::{stdin, Read};
//use std::string::String;
//use std::time::Duration;

//mod modes;

//static DATA2: phf::Map<&'static str, &str> = phf_map! {
//    "hallo" => "bye",
//    "bye" => "hallo",
//    "123" => "456",
//};

//include!("../data/codegens.rs");

//fn main() {
//    //    let data = load_map();
//    //    generate_map();
//    //    let data = load_data();
//    //    for (k,v) in DATA.to_iter() {
//    //        println!("key: {k}, value: {v}");
//    //    }
//    //    for DATA in DATAARR.entries().iter() {
//    for i in 0..DATAARR.len() {
//        println!("{:?}", DATAARR[i]);
//    }
//    let mut r: String = String::new();
//    loop {
//        stdin().read_line(&mut r).unwrap();
//        println!("{:?}", DATAA.get(r.trim()));
//        r = "".to_owned();
//        r = "".to_owned();
//    }
//    let _raw_mode: RawMode = RawMode::enable();
//    let init_state: BufferState = BufferState {
//        text: "".to_owned(),
//        position: (1, 1),
//    };
//    let mut buffer_state = init_state;
//    let mut command: Command = Command::SwichMode(Mode::Normal);
//    while command != Command::Exit {
//        match command {
//            Command::SwichMode(Mode::Normal) => {
//                let (c, s) = normal_mode(buffer_state);
//                buffer_state = s;
//                command = c;
//            }
//            Command::SwichMode(Mode::Insert) => {
//                let (c, s) = insert_mode(buffer_state);
//                buffer_state = s;
//                command = c;
//            }
//            Command::SwichMode(Mode::Steno) => {
//                let (c, s) = steno_mode(buffer_state);
//                buffer_state = s;
//                command = c;
//            }
//            _ => {
//                command = Command::Exit;
//            }
//        }
//    }
//}
//
//fn steno_mode(mut buffer_state: BufferState) -> (Command, BufferState) {
//    //    let mut keys = Vec::new();
//    //    while keys != vec![Keycode::Escape] {
//    //    if poll(Duration::from_millis(500)).unwrap() {
//    //        if keys != DeviceState::new().get_keys() {
//    //            for key in &keys {
//    //                stdout().execute(style::Print(key));
//    //            }
//    //            keys = DeviceState::new().get_keys();
//    //        }
//    //    }
//    //    }
//    while !DeviceState::new().get_keys().contains(&Keycode::S) {}
//    loop {
//        let mut keys: HashSet<Keycode> = HashSet::new();
//        let mut new_keys = DeviceState::new().get_keys();
//        while new_keys != vec![] {
//            for key in &new_keys {
//                keys.insert(*key);
//            }
//            new_keys = DeviceState::new().get_keys();
//        }
//        if keys.contains(&Keycode::Escape) {
//            return (
//                Command::SwichMode(Mode::Normal),
//                buffer_state.update_position(),
//            );
//        } else {
//            for key in keys {
//                stdout().execute(style::Print(key)).unwrap();
//            }
//        }
//    }
//}
//
//fn is_key_press(e: &crossterm::event::Event, key: char) -> bool {
//    if let crossterm::event::Event::Key(k) = e {
//        if let crossterm::event::KeyCode::Char(c) = k.code {
//            if c == key {
//                return true;
//            }
//        }
//    }
//    false
//}
//
//fn get_char(e: &crossterm::event::Event) -> Option<char> {
//    if let crossterm::event::Event::Key(k) = e {
//        if let crossterm::event::KeyCode::Char(c) = k.code {
//            return Some(c);
//        }
//    }
//    None
//}
//
//fn refresh() {
//    stdout()
//        .execute(MoveRight(1))
//        .unwrap()
//        .execute(MoveLeft(1))
//        .unwrap();
//}
//
//impl BufferState {
//    fn update_position(mut self) -> BufferState {
//        self.position = crossterm::cursor::position().unwrap();
//        self
//    }
//}
//
//fn normal_mode(mut bstate: BufferState) -> (Command, BufferState) {
//    draw_frame();
//    while poll(Duration::from_millis(500)).unwrap() {
//        crossterm::event::read();
//    }
//    let mut stdout = stdout();
//    stdout
//        .execute(DisableBlinking)
//        .unwrap()
//        .execute(cursor::SetCursorShape(CursorShape::Block))
//        .unwrap();
//    stdout
//        .execute(MoveTo(bstate.position.0, bstate.position.1))
//        .unwrap();
//    loop {
//        if poll(Duration::from_millis(500)).unwrap() {
//            let event: crossterm::event::Event = crossterm::event::read().unwrap();
//            if is_key_press(&event, 'q') {
//                return (Command::Exit, bstate);
//            }
//            if let Some(c) = get_char(&event) {
//                match c {
//                    'h' => {
//                        stdout.execute(MoveLeft(1)).unwrap();
//                    }
//                    'j' => {
//                        stdout.execute(MoveDown(1)).unwrap();
//                    }
//                    'k' => {
//                        stdout.execute(MoveUp(1)).unwrap();
//                    }
//                    'l' => {
//                        stdout.execute(MoveRight(1)).unwrap();
//                    }
//                    '0' => {
//                        stdout
//                            .execute(MoveTo(1, crossterm::cursor::position().unwrap().1))
//                            .unwrap();
//                    }
//                    '$' => {
//                        stdout
//                            .execute(MoveTo(
//                                crossterm::terminal::size().unwrap().0 - 2,
//                                crossterm::cursor::position().unwrap().1,
//                            ))
//                            .unwrap();
//                    }
//                    'i' => return (Command::SwichMode(Mode::Insert), bstate.update_position()),
//                    's' => return (Command::SwichMode(Mode::Steno), bstate.update_position()),
//                    _ => {}
//                }
//            }
//        }
//    }
//}
//
//fn load_data() -> HashMap<String, String> {
//    serde_json::from_str::<HashMap<String, Value>>(
//        &read_to_string("./data/palantype-DE.json").unwrap(),
//    )
//    .unwrap()
//    .into_iter()
//    .map(|(x, y): (String, Value)| {
//        if let Value::String(s) = y {
//            (x, s)
//        } else {
//            panic!("JSON");
//        }
//    })
//    .collect()
//}

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
//
//

use std::collections::HashMap;

use loading::load_dictionary::Stroke;
use loading::load_dictionary::read_file;

use rendering::setup_terminal::RawMode;


mod loading;
mod rendering;

fn main() {
    let _dic = loading();
    let _raw_mode = setup_terminal();
}


fn loading() -> HashMap<Vec<Stroke>, String> {
   println!("Loading Dictionary");
   let dic = read_file("data/dictionary.json");  
   println!("Dictionary loaded!");
   dic
}


fn setup_terminal() -> RawMode {
    RawMode::enable()
}



