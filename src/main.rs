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

    fn build(&mut self, words: &[&str]) {
        let mut last = "";
        let mut forks = vec![TNode::new(0, last, false)];

        let mut ptr = 1;
        self.nodes.push(0);

        for (i, word) in words.iter().cloned().chain(iter::once("")).enumerate() {
            println!("{}, {}", i, word);
        }
    }
}

fn main() {
    let mut words = vec!["auto", "autobus", "brno", "brnena", "autori"];
    let t = Trie::new(&mut words);
}
