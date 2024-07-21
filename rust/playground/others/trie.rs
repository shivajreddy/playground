use std::collections::HashMap;


struct Node {
    children: HashMap<char, Node>,
    end_of_word: bool
}

impl Node {
    fn new() -> Self {
        Self {children: HashMap::new(), end_of_word: false}
    }
}

struct Trie {
    root: Node
}

impl Trie {
    
    fn new() -> Self {
        Self { root: Node::new() }
    }
    
    fn insert(&mut self, word: String) {
        let mut curr_node = *self.root;

        for curr_char in word.chars() {
            if !(&curr_node.children.contains_key(&curr_char)) {
                curr_node.children.insert(curr_char, Node::new());
            }
            curr_node = &mut curr_node.children.get(&curr_char).unwrap();
        }
    }
    // fn sesarch(&self, word: String) -> bool {}
    // fn starts_with(&self, prefix: String) -> bool {}
}

