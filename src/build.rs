use std::{env, io::BufWriter, fs::File, path::Path, collections::HashMap};
use phf_codegen;
use serde_json::Value;
use std::fs::read_to_string;
use std::io::{Write};
use std::collections::{HashSet};


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUSTFLAGS","-C link-args=-Wl, -zstack-size=100000000");
    if !std::path::Path::new("./data/codegens.rs").exists() {
        generate();
    }
}

fn convert(c: char) -> String {
    match c {
        'ʃ' => "SCH".to_owned(),
        'Ä' => "AE".to_owned(),
        'Ö' => "OE".to_owned(),
        'Ü' => "UE".to_owned(),
        '+' => "P".to_owned(),
        '~' => "T".to_owned(),
        '-' => "MI".to_owned(),
        x   => format!("{c}")
    }
}

fn generate() {
    env::set_var("OUT_DIR", "data");
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegens.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());
    let data = load_data();
    let mut map = phf_codegen::Map::<String>::new();
    let mut set: HashMap::<char,phf_codegen::Map<String>> = HashMap::new();
    for (inp,outp) in data.iter() {
        let c: char  = inp.chars().into_iter().next().unwrap();
        if set.contains_key(&c) {
            let out: String = format!("\"{}\"",outp);
            set.get_mut(&c).unwrap().entry(inp.to_string(),&out);
        } else {
            set.insert(c,phf_codegen::Map::<String>::new());
            let out: String = format!("\"{}\"",outp);
            set.get_mut(&c).unwrap().entry(inp.to_string(),&out);
        }
    }
    for (c,map) in set.into_iter() {
    writeln!(
        &mut file,
        "static DATA{}: phf::Map<&'static str, &str> = \n{};\n",
        convert(c),
        map.build()).unwrap()
    }
//    for (inp,outp) in data.iter() {
//        map.entry(inp.to_string(),outp);
//    }
//    writeln!(
//        &mut file,
//        "static DATA: phf::Map<&'static str, &str> = \n{};\n",
//        map.build()).unwrap();
}

fn load_data() -> HashMap<String, String> {
    serde_json::from_str::<HashMap<String, Value>>
        (
                include_str!("../data/phf.json")
            ) 
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
