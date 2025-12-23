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
        println!("decode");
    } else {
        println!("encode");
    }

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

    println!("\ntree: {:?}", tree);
    let mut codebook: HashMap<u8, String> = HashMap::new();
    let mut prefix: String = "".to_string();
    buildCodes(tree[0].clone(), prefix, &mut codebook);

    println!("\ncodebook: {:?}", codebook);

    let mut msg: Vec<u8> = Vec::new();
    for byte in data.clone() {
        msg.push(u8::from_str_radix(codebook.get(&byte).unwrap(), 2).unwrap());
    }
    println!("msg: {:?}", msg);

    fs::write(fComp[0].clone().to_owned() + ".huf", msg);

    return ();
}
