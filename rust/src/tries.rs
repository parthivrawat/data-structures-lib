use std::collections::HashMap;

use crate::Error;

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
    count: usize,
}

/// A prefix tree for string storage.
#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
    len: usize,
}

impl Trie {
    /// Creates a new Trie.
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
            len: 0,
        }
    }

    /// Inserts a word.
    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_default();
            node.count += 1;
        }
        if !node.is_end {
            self.len += 1;
            node.is_end = true;
        }
    }

    /// Returns true if the word is in the trie.
    pub fn search(&self, word: &str) -> bool {
        self.find(word)
            .map(|node| node.is_end)
            .unwrap_or(false)
    }

    /// Returns true if any word starts with the prefix.
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.find(prefix).is_some()
    }

    fn find(&self, word: &str) -> Option<&TrieNode> {
        let mut node = &self.root;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(next) => node = next,
                None => return None,
            }
        }
        Some(node)
    }

    /// Deletes a word.
    pub fn delete(&mut self, word: &str) -> Result<(), Error> {
        if !self.search(word) {
            return Err(Error::NotFound);
        }
        Self::delete_node(&mut self.root, word, 0);
        self.len -= 1;
        Ok(())
    }

    fn delete_node(node: &mut TrieNode, word: &str, index: usize) {
        if index == word.chars().count() {
            node.is_end = false;
            return;
        }
        let ch = word.chars().nth(index).unwrap();
        if let Some(child) = node.children.get_mut(&ch) {
            Self::delete_node(child, word, index + 1);
            child.count -= 1;
            if child.count == 0 {
                node.children.remove(&ch);
            }
        }
    }

    /// Returns all words with the given prefix.
    pub fn words(&self, prefix: &str) -> Vec<String> {
        let node = if prefix.is_empty() {
            &self.root
        } else {
            match self.find(prefix) {
                Some(n) => n,
                None => return Vec::new(),
            }
        };
        let mut result = Vec::new();
        let mut current = prefix.to_string();
        Self::collect(node, &mut current, &mut result);
        result
    }

    fn collect(node: &TrieNode, prefix: &mut String, result: &mut Vec<String>) {
        if node.is_end {
            result.push(prefix.clone());
        }
        for (ch, child) in &node.children {
            prefix.push(*ch);
            Self::collect(child, prefix, result);
            prefix.pop();
        }
    }

    /// Returns the number of words.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the trie is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}
