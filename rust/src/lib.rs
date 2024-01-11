mod iterator;
mod node;


use crate::iterator::PathTrieIterator;
use node::Node;
use std::path::{Path, PathBuf};


#[derive(Debug)]
pub struct PathTrie {
    // The following root node is not a filesystem root, but an empty path which
    // serves as a common ancestor for all path nodes, which may include
    // multiple filesystem root nodes, relative paths, etc. This was done to
    // maximize reuse of the Node logic, without having to handle special edge
    // cases here in PathTrie.
    root_node: Node,
}

impl PathTrie {
    pub fn new() -> Self {
        Self {
            root_node: Node {
                path: PathBuf::from(""),
                children: Vec::new(),
                is_element: false,
            },
        }
    }

    pub fn insert(&mut self, path: &Path) -> Result<(), String> {
        if path.as_os_str().is_empty() {
            return Err("cannot insert empty path".to_string());
        }

        self.root_node.insert(path)
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
