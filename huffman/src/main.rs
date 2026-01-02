use std::{
    cmp::Ordering,
    collections::{HashMap, hash_map},
    env,
    fs::{self, File},
    io::{BufWriter, LineWriter, Write},
    iter, vec,
};

const HEADER_FIXED: usize = 11;
const CHUNK_SIZE: usize = 8 * 1024 * 1024;

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
}

fn buildCodes(node: &Node, prefix: String, codebook: &mut HashMap<u8, String>) {
    match node.byte {
        Some(b) => {
            codebook.insert(b, prefix);
            return ();
        }
        None => {
            let left = prefix.clone() + "0";
            let right = prefix.clone() + "1";
            match &node.left {
                Some(n) => buildCodes(&n, left, codebook),
                None => (),
            }
            match &node.right {
                Some(n) => buildCodes(&n, right, codebook),
                None => (),
            }
        }
    };
}

fn encodeFreq(cb: HashMap<u8, u32>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    for (x, y) in cb {
        bytes.push(x);
        for b in y.to_be_bytes() {
            bytes.push(b);
        }
    }

    return bytes;
}

fn decode(tree: &Vec<Node>, data: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut node: &Node = &tree[0];

    println!("data size: {}", data.len());

    for bit in data {
        node = match bit {
            0 => node.left.as_deref().expect("Left node does not exist"),
            1 => node.right.as_deref().expect("Right node does not exist"),
            x => panic!("Not 0 or 1 dectected: \'{:?}\'", x),
        };

        if let Some(b) = node.byte {
            result.push(b);
            node = &tree[0];
        }
    }

    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: huf {{file}}");
        return ();
    }
    let fname = args[1].clone();
    let fComp: Vec<&str> = fname.split('.').collect();

    if **&fComp.last().unwrap() == "huf" {
        // ---------------------------------------------------------------------
        println!("Decode");

        // raw bytes
        let mut bytes: Vec<u8> = fs::read(&args[1]).expect("error opening file");

        // signature
        let sig: Vec<u8> = bytes[..2].to_vec();
        if sig != [72, 70].to_vec() {
            println!("Not a .huf file");
            return;
        }

        // padding amount
        let padding_amt: u8 = bytes[2];

        // freq table size
        let arr: [u8; 8] = bytes[3..HEADER_FIXED]
            .try_into()
            .expect("Size does not match");
        let freq_length = u64::from_be_bytes(arr);

        // freq table
        let freq_bytes: Vec<u8> =
            bytes[HEADER_FIXED..((freq_length + HEADER_FIXED as u64) as usize)].to_vec();

        // data
        let mut data: Vec<u8> = bytes[((freq_length + HEADER_FIXED as u64) as usize)..].to_vec();

        // dealloc bytes - no longer needed
        bytes = Vec::new();

        // freq_bytes -> HashMap
        let mut freq: HashMap<u8, u32> = HashMap::new();
        let mut i = 0;
        while i < freq_bytes.len() {
            let arr: [u8; 4] = freq_bytes[i + 1..i + 5]
                .try_into()
                .expect("Freq bytes mismatch");
            let code = u32::from_be_bytes(arr);
            freq.insert(freq_bytes[i], code);
            i += 5;
        }

        // rebuild tree
        let mut tree: Vec<Node> = Vec::new();
        buildTree(&mut tree, &freq);
        println!("build tree done");

        // dealloc freq - no longer needed
        freq = HashMap::new();

        let mut codebook: HashMap<u8, String> = HashMap::new();
        let prefix: String = "".to_string();
        buildCodes(&tree[0], prefix, &mut codebook);
        println!("build codes done");

        // dealloc codebook - no longer needed
        codebook = HashMap::new();
        // rebuild data
        println!("rebuilding data");

        println!("data len: {:?}", data.len());
        let mut bits: Vec<u8> = Vec::with_capacity(data.len() * 8);
        let mut bit = "".to_string();
        let mut count = 0;
        for b in &data {
            println!("count: {}", count);
            bit = format!("{:08b}", b);
            for c in bit.chars() {
                match c {
                    '0' => bits.push(0),
                    '1' => bits.push(1),
                    x => panic!("unexpected character: \'{:?}\'", x),
                }
            }
            count += 1;
        }
        bits = bits[..bits.len() - padding_amt as usize].to_vec();

        // dealloc data - no longer needed
        data = Vec::new();

        println!("decoding");
        let msg: Vec<u8> = decode(&tree, bits);
        // dealloc tree - no longer needed
        tree = Vec::new();
        // dealloc bits - no longer needed
        bits = Vec::new();

        let mut newFile: String = "".to_string();
        for n in &fComp[..fComp.len() - 2] {
            newFile.push_str(n);
            newFile.push('.');
        }
        newFile.push_str(&fComp[fComp.len() - 2]);

        let file = File::create(newFile).expect("Failed to create new file");
        let mut writer = BufWriter::with_capacity(CHUNK_SIZE, file);

        for chunk in msg.chunks(CHUNK_SIZE) {
            let _ = writer.write_all(chunk);
        }
    } else {
        // ---------------------------------------------------------------------
        println!("Encode");

        let mut data: Vec<u8> = fs::read(&args[1]).expect("error opening file");

        let mut freq: HashMap<u8, u32> = HashMap::new();

        println!("reading data");
        for x in &data {
            match freq.get(&x) {
                Some(&i) => freq.insert(*x, i + 1),
                _ => freq.insert(*x, 1),
            };
        }

        println!("building tree");
        let mut tree: Vec<Node> = Vec::new();
        buildTree(&mut tree, &freq);
        let freqBytes: Vec<u8> = encodeFreq(freq);
        // dealloc freq - no longer needed
        freq = HashMap::new();

        println!("building codes");
        let mut codebook: HashMap<u8, String> = HashMap::new();
        let prefix: String = "".to_string();
        buildCodes(&tree[0], prefix, &mut codebook);

        let mut msg: Vec<u8> = Vec::new();

        let mut temp: String = "".to_string();
        for byte in &data {
            temp.push_str(codebook.get(&byte).unwrap());
        }
        // dealloc codebook - no longer needed
        codebook = HashMap::new();
        // dealloc data - no longer needed
        data = Vec::new();

        let padding = (8 - temp.len() % 8) % 8;

        // padding data
        let mut pad = "".to_string();
        for _ in 0..padding {
            pad.push('0');
        }
        temp.push_str(&pad);

        // 2 Byte signature
        msg.append(&mut "HF".to_string().into_bytes());

        // 1 Byte padding amount
        msg.push(padding as u8);

        // 8 Byte Freq Size
        let length = (freqBytes.len() as u64).to_be_bytes();
        for b in length {
            msg.push(b);
        }

        // x Byte Freq Table
        for b in freqBytes {
            msg.push(b);
        }

        // padded data
        for i in (0..temp.len()).step_by(8) {
            let n = &temp[i..i + 8];
            msg.push(u8::from_str_radix(n, 2).unwrap());
        }

        println!("writing to file");
        // do buffered writes
        let newFile = fname.to_owned() + ".huf";

        let file = File::create(newFile).expect("Failed to create new file");
        let mut writer = BufWriter::with_capacity(CHUNK_SIZE, file);

        for chunk in msg.chunks(CHUNK_SIZE) {
            let _ = writer.write_all(chunk);
        }
    }

    return ();
}
