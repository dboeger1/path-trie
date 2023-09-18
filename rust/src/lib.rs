mod iterator;
mod node;


use crate::iterator::PathTrieIterator;
use node::Node;
use std::path::{Path, PathBuf};


#[derive(Debug)]
pub struct PathTrie {
    root_nodes: Vec<Node>,
}

impl PathTrie {
    pub fn new() -> Self {
        Self {
            root_nodes: vec![Node {
                path: PathBuf::from("/"),
                children: Vec::new(),
            }],
        }
    }

    pub fn insert(&mut self, path: &Path) -> Result<(), String> {
        for root_node in self.root_nodes.iter_mut() {
            root_node.insert(path)?;
        }
        Ok(())
    }

    pub fn iter(&self) -> PathTrieIterator {
        PathTrieIterator::new(self)
    }
}

impl Default for PathTrie {
    fn default() -> Self {
        Self::new()
    }
}
