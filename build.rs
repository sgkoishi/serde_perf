use rand::Rng;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

fn main() {
    let mut rng = rand::thread_rng();
    let r = include_str!("./count").trim().parse::<usize>().unwrap();
    let do_ser = Path::new("./serialize").exists();
    let w = File::create("./src/definitions.rs").unwrap();
    let mut w = BufWriter::new(w);

    if do_ser {
        w.write("use serde::{Deserialize, Serialize};\n\n".as_bytes())
            .unwrap();
    }
    w.write("#[allow(non_camel_case_types)]\n".as_bytes())
        .unwrap();
    if do_ser {
        w.write("#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize, Clone\n)]\n".as_bytes())
                .unwrap();
    } else {
        w.write("#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone\n)]\n".as_bytes())
            .unwrap();
    }
    w.write("pub enum Ids {\n".as_bytes()).unwrap();
    let mut hs = HashSet::new();
    while hs.len() < r {
        hs.insert(format!(
            "    {}{}{}_{}_{:0>4},\n",
            rng.gen_range(65..91) as u8 as char,
            rng.gen_range(65..91) as u8 as char,
            rng.gen_range(65..91) as u8 as char,
            rng.gen_range(65..91) as u8 as char,
            rng.gen_range(0..10000)
        ));
    }
    for i in hs {
        w.write(i.as_bytes()).unwrap();
    }
    w.write("}".as_bytes()).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=count");
}
