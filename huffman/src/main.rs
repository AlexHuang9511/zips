use std::{collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 2 {
        println!("Usage: huf {{file}}");
        return ();
    }

    let data: Vec<u8> = fs::read(&args[1]).expect("error opening file");
    println!("{:?}", data);

    let mut dict: HashMap<u8, u32> = HashMap::new();

    for x in data {
        match dict.get(&x) {
            Some(&i) => dict.insert(x, i + 1),
            _ => dict.insert(x, 1),
        };
    }

    println!("Dict: {:?}", dict);

    return ();
}
