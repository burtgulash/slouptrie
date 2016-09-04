use std::iter;

struct TNode<'a> {
    prefix_size: usize,
    term: &'a str,
    children: Vec<TNode<'a>>,
    ptr: usize,
    is_terminal: bool,
}

impl<'a> TNode<'a> {
    fn new(prefix_size: usize, term: &'a str, is_terminal: bool) -> TNode<'a> {
        TNode {
            prefix_size: prefix_size,
            term: term,
            children: Vec::new(),
            ptr: 0,
            is_terminal: is_terminal,
        }
    }
}

fn common_prefix_size(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars())
        .take_while(|&(ac, bc)| {ac == bc})
        .fold(0, |acc, (x, _)| acc + x.len_utf8())
}

struct Trie {
    terms: Vec<u8>,
    firsts: Vec<char>,
    termlens: Vec<usize>,
    nodes: Vec<usize>,
    children: Vec<usize>,
    is_terminal: Vec<bool>,
}

impl Trie {
    fn new(words: &mut [&str]) -> Trie {
        words.sort();
        let mut t = Trie {
            terms: Vec::new(),
            firsts: Vec::new(),
            termlens: Vec::new(),
            nodes: Vec::new(),
            children: Vec::new(),
            is_terminal: Vec::new(),
        };
        t.build(words);
        t
    }

    fn flush_node(&mut self, node: &TNode, term: &str) {
        self.children.push(node.ptr);
        self.firsts.push(term.chars().next().unwrap());
        self.termlens.push(term.len());
        self.terms.extend(term.as_bytes());
        self.is_terminal.push(node.is_terminal);
    }

    fn build(&mut self, words: &[&str]) {
        let mut last = "";
        let mut stack = vec![TNode::new(0, last, false)];

        let mut ptr = 1;
        self.nodes.push(0);

        for word in words.iter().cloned().chain(iter::once("")) {
            let prefix_size = common_prefix_size(word, last);
            if prefix_size < last.len() {
                let mut flushed = stack.pop().unwrap();
                while prefix_size < stack[stack.len() - 1].prefix_size {
                    //println!("{}", flushed.term);
                    let mut parent = stack.pop().unwrap();
                    parent.children.push(flushed);
                    flushed = parent;

                    for ch in &flushed.children {
                        let term = &ch.term[flushed.prefix_size .. ch.prefix_size];
                        self.flush_node(ch, term);
                    }
                    self.nodes.push(flushed.children.len());
                    flushed.ptr = ptr;
                    ptr += 1;
                }

                if prefix_size > stack[stack.len() - 1].prefix_size {
                    stack.push(TNode::new(prefix_size, word, false));
                }

                let l = stack.len() - 1;
                stack[l].children.push(flushed);
            }

            stack.push(TNode::new(word.len(), word, true));
            last = word;
        }

        stack.pop();
        let root = stack.pop().unwrap();
        for ch in &root.children {
            let term = &ch.term[.. ch.prefix_size];
            self.flush_node(ch, term);
            println!("ROOT CHILD: {}", ch.term);
        }
        self.nodes.push(root.children.len());
    }
}

fn main() {
let mut ws = vec![
    "auto",
    "autobus",
    "auta",
    "auatky",
    "asiat",
    "autor",
    "atom",
    "autorky",
    "ati",
    "ararat",
    "ataturk",
    "autista",
    "auty",
    "burani",
    "burky",
    "burrow",
    "borrow",
    "buráci",
    "zmrdi",
    "záledí",
    "zbrna",
    "zbraně",
    "bobry",
    "bobcat",
    "bobani",
    "zobr",
    "zlobr",
    "zulu",
    "zubřice",
    "zuby",
    "zálezí",
];

    let mut words = vec!["auto", "autobus", "brno", "brnena", "autori"];
    let t = Trie::new(&mut ws);
    println!("{:?}", t.is_terminal);
    println!("{:?}", t.termlens);
    println!("{:?}", t.firsts);
    println!("{:?}", t.terms);
    println!("{:?}", t.nodes);
    println!("{:?}", t.children);
}
