use std::collections::BTreeMap;

struct Trie {
    children: BTreeMap<char, Trie>,
    is_end_of_word: bool,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Trie {
            children: BTreeMap::new(),
            is_end_of_word: false
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut cur_node = self;
        for ch in word.chars() {
            cur_node = cur_node.children.entry(ch).or_insert(Trie::new());
        }
        cur_node.is_end_of_word = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut cur_node = self;
        for ch in word.chars() {
            match cur_node.children.get(&ch) {
                Some(node) =>{
                    cur_node = node;
                }
                None => {
                    return false;
                }
            }
        }
        cur_node.is_end_of_word
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut cur_node = self;
        for ch in prefix.chars() {
            match cur_node.children.get(&ch) {
                Some(node) =>{
                    cur_node = node;
                }
                None => {
                    return false;
                }
            }
        }
        true
    }
}



fn main() {
    let mut trie = Trie::new();

    trie.insert("apple".to_string());
    trie.insert("app".to_string());
    trie.insert("banana".to_string());

    println!("Searching 'apple': {}", trie.search("apple".to_string())); // Should print true
    println!("Searching 'app': {}", trie.search("app".to_string())); // Should print true
    println!("Searching 'apples': {}", trie.search("apples".to_string())); // Should print false

    println!("Prefix 'app' exists: {}", trie.starts_with("app".to_string())); // Should print true
    println!("Prefix 'ban' exists: {}", trie.starts_with("ban".to_string())); // Should print true
    println!("Prefix 'cherry' exists: {}", trie.starts_with("cherry".to_string())); // Should print false
}
