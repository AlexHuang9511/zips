use std::{cmp::Ordering, collections::HashMap, env, fs};

#[derive(Debug, Clone)]
struct Node {
    freq: u32,
    byte: Option<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new() -> Node {
        Node {
            freq: 0,
            byte: None,
            left: None,
            right: None,
        }
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq.cmp(&other.freq).then(self.byte.cmp(&other.byte))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}
impl Eq for Node {}

fn buildTree(tree: &mut Vec<Node>, freq: &HashMap<u8, u32>) {
    for (x, f) in freq {
        tree.push(Node {
            freq: *f,
            byte: Some(*x),
            left: None,
            right: None,
        });
    }
    tree.sort();
    println!("list: {:?}\n", tree);

    while tree.len() > 1 {
        let l = tree.remove(0).clone();
        let r = tree.remove(0).clone();
        tree.push(Node {
            freq: l.freq + r.freq,
            byte: None,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        });
        tree.sort();
    }
    println!("list: {:?}", tree);
}

fn buildCodes(node: Node, prefix: String, codebook: &mut HashMap<u8, String>) {
    match node.byte {
        Some(b) => {
            codebook.insert(b, prefix);
            return ();
        }
        None => {
            let left = prefix.clone() + "0";
            let right = prefix.clone() + "1";
            match node.left {
                Some(n) => buildCodes(*n, left, codebook),
                None => (),
            }
            match node.right {
                Some(n) => buildCodes(*n, right, codebook),
                None => (),
            }
        }
    };
}

fn encodeBook(cb: HashMap<u8, u32>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    for (x, y) in cb {
        bytes.push(x);
        for b in y.to_be_bytes() {
            bytes.push(b);
        }
    }

    return bytes;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 2 {
        println!("Usage: huf {{file}}");
        return ();
    }
    let fname = args[1].clone();
    let fComp: Vec<&str> = fname.split('.').collect();

    if *fComp.last().unwrap() == "huf" {
        // ---------------------------------------------------------------------
        println!("decode");

        // raw bytes
        let bytes: Vec<u8> = fs::read(&args[1]).expect("error opening file");
        println!("{:?}", bytes);

        // freq table size
        let arr: [u8; 4] = bytes[2..6].try_into().expect("Size does not match");
        let freq_length = u32::from_be_bytes(arr);
        println!("length: {:?}", freq_length);

        // freq table
        let freq_bytes: Vec<u8> = bytes[6..((freq_length + 6) as usize)].to_vec();
        println!("freq_bytes: {:?}", freq_bytes);

        // data
        let data: Vec<u8> = bytes[((freq_length + 6) as usize)..].to_vec();
        println!("data: {:?}", data);

        // freq_bytes -> HashMap
        // rebuild tree
        // rebuild codebook
        // rebuild data
    } else {
        // ---------------------------------------------------------------------
        println!("encode");

        let data: Vec<u8> = fs::read(&args[1]).expect("error opening file");
        println!("{:?}", data);

        let mut freq: HashMap<u8, u32> = HashMap::new();

        for x in data.clone() {
            match freq.get(&x) {
                Some(&i) => freq.insert(x, i + 1),
                _ => freq.insert(x, 1),
            };
        }

        println!("Freq: {:?}", freq);
        let mut tree: Vec<Node> = Vec::new();
        buildTree(&mut tree, &freq);
        let freqBytes: Vec<u8> = encodeBook(freq);
        println!("\ncbBytes: {:?}", freqBytes);

        println!("\ntree: {:?}", tree);
        let mut codebook: HashMap<u8, String> = HashMap::new();
        let prefix: String = "".to_string();
        buildCodes(tree[0].clone(), prefix, &mut codebook);

        println!("\ncodebook: {:?}", codebook);
        let mut msg: Vec<u8> = Vec::new();
        msg.append(&mut "HF".to_string().into_bytes()); // 2 Byte Header

        let length = (freqBytes.len() as u32).to_be_bytes(); // 4 Byte Freq Size
        for b in length {
            msg.push(b);
        }

        // x Byte Freq Table
        for b in freqBytes {
            msg.push(b);
        }

        for byte in data.clone() {
            msg.push(u8::from_str_radix(codebook.get(&byte).unwrap(), 2).unwrap());
        }
        println!("msg: {:?}", msg);

        let newFile = fComp[0].to_owned() + ".huf";

        let _ = fs::write(&newFile, msg);
    }

    return ();
}
