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
        self.freq.cmp(&other.freq)
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

fn buildTree(tree: Node, freq: &HashMap<u8, u32>) {
    let mut v: Vec<Node> = Vec::new();
    for (x, f) in freq {
        v.push(Node {
            freq: *f,
            byte: Some(*x),
            left: None,
            right: None,
        });
    }
    v.sort();

    while v.len() > 1 {
        let l = v.remove(0).clone();
        let r = v.remove(0).clone();
        v.push(Node {
            freq: l.freq + r.freq,
            byte: None,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        });
        v.sort();
    }
    println!("list: {:?}", v);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 2 {
        println!("Usage: huf {{file}}");
        return ();
    }

    let data: Vec<u8> = fs::read(&args[1]).expect("error opening file");
    println!("{:?}", data);

    let mut freq: HashMap<u8, u32> = HashMap::new();

    for x in data {
        match freq.get(&x) {
            Some(&i) => freq.insert(x, i + 1),
            _ => freq.insert(x, 1),
        };
    }

    println!("Freq: {:?}", freq);
    let mut tree: Node = Node::new();
    buildTree(tree, &freq);

    return ();
}
