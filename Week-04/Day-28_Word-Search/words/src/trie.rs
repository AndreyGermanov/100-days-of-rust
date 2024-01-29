use std::collections::BTreeMap;

pub struct Trie {
    pub children: BTreeMap<u8, Box<Trie>>,
    pub is_end: bool,
}

impl Trie {

    pub fn new() -> Self {
        Trie {
            children: BTreeMap::new(),
            is_end: false,
        }
    }

    pub  fn insert(&mut self, word: &[u8]) -> bool {
        let mut current = self;
        for ch in word {
            current = current.children.entry(*ch)
                .or_insert(Box::new(Trie::new()));
        }

        let inserted = current.is_end == false;
        current.is_end = true;
        return inserted;
    }

    pub fn is_empty(&self) -> bool {
        !self.is_end && self.children.is_empty()
    }
}