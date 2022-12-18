use std::{env, io::BufWriter, fs::File, path::Path, collections::HashMap};
use phf_codegen;
use serde_json::Value;
use std::fs::read_to_string;
use std::io::{Write};


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUSTFLAGS","-C link-args=-Wl, -zstack-size=100000000");
    if !std::path::Path::new("./data/codegens.rs").exists() {
        generate();
    }
}

fn generate() {
    env::set_var("OUT_DIR", "data");
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegens.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());
    let data = load_data();
//    let mut map = phf_codegen::Map::<String,String,"../data/codegens.rs">;
//    let mut map = phf_codegen::Map {keys: String, values: String, path: "../data/codegens.rs".to_owned()};
//    let mut map = phf_codegen::Map::<&[u8]>::new();
    let mut map = phf_codegen::Map::<String>::new();
    for (inp,outp) in data.into_iter() {
        let out: String = format!("\"{}\"",outp);
        map.entry(inp,&out);
    }
    writeln!(
        &mut file,
        "static DATA: phf::Map<&'static str, &str> = \n{};\n",
        map.build()).unwrap();
}

//fn load_data() -> HashMap<String, String> {
//    HashMap::new()
//}
fn load_data() -> HashMap<String, String> {
    serde_json::from_str::<HashMap<String, Value>>
        (
//            &read_to_string("./data/palantype-DE.json").unwrap()
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
