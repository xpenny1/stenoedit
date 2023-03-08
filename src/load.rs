//struct stroke {
//    v: bool,
//    s: bool
//}

use std::{collections::HashMap, fs::read_to_string, io::stdin};

#[allow(dead_code)]
#[derive(Debug)]
enum Key {
    v = (1 << 0),
    Dl = (1 << 1),
    b = (1 << 2),
    SCHl = (1 << 3),
    Sl = (1 << 4),
    Fl = (1 << 5),
    Gl = (1 << 6),
    Nl = (1 << 7),
    Bl = (1 << 8),
    Ml = (1 << 9),
    PLUSl = (1 << 10),
    Ll = (1 << 11),
    Ä = (1 << 12),
    E = (1 << 13),
    A = (1 << 14),
    TILDE = (1 << 15),
    U = (1 << 16),
    I = (1 << 17),
    O = (1 << 18),
    Ü = (1 << 19),
    Mr = (1 << 20),
    PLUSr = (1 << 21),
    Lr = (1 << 22),
    Gr = (1 << 23),
    Nr = (1 << 24),
    Br = (1 << 25),
    SCHr = (1 << 26),
    Sr = (1 << 27),
    Fr = (1 << 28),
    n = (1 << 29),
    Dr = (1 << 30),
    s = (1 << 31),
}

type Stroke = u32;

type Entry = (Vec<Stroke>, String);

fn string_to_stroke(s: &str) -> Stroke {
    let mut i = 0;
    let mut stroke = 0;
    let cs: Vec<char> = s.chars().collect();
    while cs.len() > i && !"-ÄEA~UIOÜ".contains(cs[i]) && "vDbʃSFGNBM+L".contains(cs[i]) {
        //match left
        match cs[i] {
            'v' => {
                stroke = stroke | Key::v as u32;
            }
            'D' => {
                stroke = stroke | Key::Dl as u32;
            }
            'b' => {
                stroke = stroke | Key::b as u32;
            }
            'ʃ' => {
                stroke = stroke | Key::SCHl as u32;
            }
            'S' => {
                stroke = stroke | Key::Sl as u32;
            }
            'F' => {
                stroke = stroke | Key::Fl as u32;
            }
            'G' => {
                stroke = stroke | Key::Gl as u32;
            }
            'N' => {
                stroke = stroke | Key::Nl as u32;
            }
            'B' => {
                stroke = stroke | Key::Bl as u32;
            }
            'M' => {
                stroke = stroke | Key::Ml as u32;
            }
            '+' => {
                stroke = stroke | Key::PLUSl as u32;
            }
            'L' => {
                stroke = stroke | Key::Ll as u32;
            }
            c => {
                println!("LEFT");
                let s: String = cs.iter().collect();
                println!("Stroke: {s}");
                panic!("invalid key: {c}");
            }
        }
        i = i + 1;
    }
    while cs.len() > i && "-ÄEA~UIOÜ".contains(cs[i]) {
        //match middle
        match cs[i] {
            'Ä' => {
                stroke = stroke | Key::Ä as u32;
            }
            'E' => {
                stroke = stroke | Key::E as u32;
            }
            'A' => {
                stroke = stroke | Key::A as u32;
            }
            '~' => {
                stroke = stroke | Key::TILDE as u32;
            }
            'U' => {
                stroke = stroke | Key::U as u32;
            }
            'I' => {
                stroke = stroke | Key::I as u32;
            }
            'O' => {
                stroke = stroke | Key::O as u32;
            }
            'Ü' => {
                stroke = stroke | Key::Ü as u32;
            }
            '-' => {}
            c => {
                println!("MIDDLE");
                let s: String = cs.iter().collect();
                println!("Stroke: {s}");
                panic!("invalid key: {c}");
            }
        }
        i = i + 1;
    }
    while cs.len() > i {
        //match end
        match cs[i] {
            'M' => {
                stroke = stroke | Key::Mr as u32;
            }
            '+' => {
                stroke = stroke | Key::PLUSr as u32;
            }
            'L' => {
                stroke = stroke | Key::Lr as u32;
            }
            'G' => {
                stroke = stroke | Key::Gr as u32;
            }
            'N' => {
                stroke = stroke | Key::Nr as u32;
            }
            'B' => {
                stroke = stroke | Key::Br as u32;
            }
            'ʃ' => {
                stroke = stroke | Key::SCHr as u32;
            }
            'S' => {
                stroke = stroke | Key::Sr as u32;
            }
            'F' => {
                stroke = stroke | Key::Fr as u32;
            }
            'n' => {
                stroke = stroke | Key::n as u32;
            }
            'D' => {
                stroke = stroke | Key::Dr as u32;
            }
            's' => {
                stroke = stroke | Key::s as u32;
            }
            c => {
                println!("END");
                let s: String = cs.iter().collect();
                println!("Stroke: {s}");
                panic!("invalid key: {c}");
            }
        }
        i = i + 1;
    }
    stroke
}

fn string_to_strokes(s: &str) -> Vec<Stroke> {
    s.split('/').map(string_to_stroke).collect()
}

fn string_to_entry(s: &str) -> Entry {
    let mut xs = s.split('\"');
    (
        {
            xs.next();
            string_to_strokes(xs.next().unwrap())
        },
        {
            xs.next();
            xs.next().unwrap().to_owned()
        },
    )
}

fn read_file() -> HashMap<Vec<Stroke>, String> {
    let mut s = read_to_string("palantype-DE.json").unwrap();
    let str = &s[0..s.len() - 1];
    //    print!("{str}");
    str.lines()
        .filter(|l| l.len() > 4)
        .map(|s| {
            //            println!("{s}");
            //            println!("{:?}", string_to_entry(s));
            string_to_entry(s)
        })
        .collect()
    //    Vec::new()
}

fn main() {
    println!("\"SNAL/LO\": \"Hallo\"");
    println!("{:?}", string_to_entry("\"SNAL/LO\": \"Hallo\""));
    let dic = read_file();
    println!("Finished");
    let mut l: String = String::new();
    loop {
        stdin().read_line(&mut l).unwrap();
        println!("{:?}", dic.get(&string_to_strokes(l.trim())));
        //        println!(
        //            "{:?}",
        //            dic.iter()
        //                .find(|(e, _)| *e == string_to_strokes(&(l.trim())))
        //        );
        l = "".to_owned();
        l = "".to_owned();
    }
}
