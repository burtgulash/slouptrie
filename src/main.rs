use std::{iter,str};
use std::cmp::min;
use std::usize;

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

fn cum_encode(xs: &[usize]) -> Vec<usize> {
    let mut cum = 0usize;
    let mut res = Vec::new();
    for x in xs {
        res.push(cum);
        cum += *x;
    }
    res.push(cum);
    res
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
        t.termlens = cum_encode(&t.termlens);
        t.nodes = cum_encode(&t.nodes);

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

    fn search(&self, mut word: &str, search_prefix: bool) -> bool {
        let nl = self.nodes.len();
        let (mut ch0, mut ch1) = (self.nodes[nl - 2], self.nodes[nl - 1]);
        loop {
            let c = word.chars().next().unwrap();
            let ci = ch0 + match self.firsts[ch0 .. ch1].binary_search(&c) {
                Ok(x) => x,
                Err(_) => break,
            };

            // println!("CI: {}, ch0: {}, ch1: {}", ci, ch0, ch1);
            let (t0, t1) = (self.termlens[ci], self.termlens[ci + 1]);
            let matched = str::from_utf8(&self.terms[t0 .. t1]).unwrap();

            //println!("matched len: {},matched: {}", matched.len(), matched);
            //println!("WORD: {}, matched: {}", &word[.. min(word.len(), matched.len())], matched);

            if word.len() < matched.len() {
                if word != &matched[..word.len()] {
                    return false;
                }
                return search_prefix;
            } else if word.len() == matched.len() {
                if word != matched {
                    return false;
                }
                return search_prefix || self.is_terminal[ci];
            }

            word = &word[matched.len() ..];
            let node = self.children[ci];
            if node == 0 {
                return false;
            }

            ch0 = self.nodes[node];
            ch1 = self.nodes[node + 1];
        }

        false
    }

    fn search_fuzzy(&self, word: &str, k: usize, search_prefix: bool)
        -> Vec<(String, usize)>
    {
        let l = word.len();
        let mut d_matrix = vec![0; (l + 1) * (l + 1)];
        for i in 0 .. l + 1 {
            d_matrix[i] = i;
        }

        let nl = self.nodes.len();
        let (ch0, ch1) = (self.nodes[nl - 2], self.nodes[nl - 1]);

        let mut matches = Vec::new();
        for c in ch0 .. ch1 {
            let wbuf = Vec::<char>::new();
            let w = word.chars().collect::<Vec<_>>();
            self._fuzzy(&mut matches, w, k, search_prefix,
                        wbuf, c, l + 1, &mut d_matrix[..], 1);
        }
        matches
    }

    fn _fuzzy(&self, matches: &mut Vec<(String, usize)>,
              word: Vec<char>, k: usize, search_prefix: bool,
              mut word_sofar: Vec<char>, node: usize,
              s: usize, mat: &mut [usize], lvl: usize)
    {
        let (t0, t1) = (self.termlens[node], self.termlens[node + 1]);
        let matched = str::from_utf8(&self.terms[t0 .. t1]).unwrap();

        assert!(matched.len() > 0);
        word_sofar.extend(matched.chars());

        let maxrow = min(lvl + matched.len(), word.len() + 1);
        for j in lvl .. maxrow {
            let mut row_min = usize::MAX;
            mat[j * s + 0] = j;

            for i in 1 .. s {
                let substituted = if word[i - 1] == word_sofar[j - 1]
                                  {0} else {1};
                let replace = mat[(j - 1) * s + i - 1] + substituted;
                let insert = mat[j * s + (i - 1)] + 1;
                let delete = mat[(j - 1) * s + i] + 1;
                let cost = min(replace, min(insert, delete));

                mat[j * s + i] = cost;
                row_min = min(row_min, cost);
            }

            // if all elements in the row exceed bound k, then there is
            // no way new matches could be formed
            if row_min > k {
                return;
            }
        }

        let mut distance = mat[maxrow * s - 1]; // Last item in maxrow

        // If you're not searching with prefix, then add to distance
        // number of remaining characters
        if !search_prefix && word_sofar.len() > word.len() {
            distance += word_sofar.len() - word.len();
        }

        if distance <= k && self.is_terminal[node] {
            matches.push( (
                word_sofar.iter().cloned().collect::<String>(),
                distance
            ) );
        }

        // find minimum in maxrow row
        let row_min = *mat[(maxrow - 1) * s .. (maxrow) * s]
                      .iter().min().unwrap();

        // if there is a way new matches can be formed
        // (row_min is within bounds of k), search children nodes
        if row_min <= k {
            let node = self.children[node];
            if node == 0 {
                return;
            }

            let (ch0, ch1) = (self.nodes[node], self.nodes[node + 1]);
            for c in ch0 .. ch1 {
                self._fuzzy(matches, word.clone(), k, search_prefix,
                            word_sofar.clone(), c, s, mat, maxrow);
            }
        }
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

    let _words = vec!["auto", "autobus", "brno", "brnena", "autori"];
    //ws = words;
    let t = Trie::new(&mut ws);
    println!("{:?}", t.is_terminal);
    println!("{:?}", t.termlens);
    println!("{:?}", t.firsts.iter().cloned().collect::<String>());
    println!("{:?}", str::from_utf8(&t.terms[..]).unwrap());
    println!("{:?}", t.nodes);
    println!("{:?}", t.children);

    println!("SEARCHING");
    let prefix_search = false;
    for w in ws {
        println!("SEARCH(prefix={:?}) {}: {}", prefix_search, w, t.search(&w[..min(w.len(),3)], prefix_search));
    }

    let q = "autor";
    let k = 2;
    println!("FUZZY search for {} with k={}", q, k);
    for (word, dist) in t.search_fuzzy(q, k, true) {
        println!("matched: {}, d={}", word, dist);
    }
}
