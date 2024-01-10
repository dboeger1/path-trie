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
            root_nodes: Vec::new(),
        }
    }

    // pub fn contains(&self, path: &Path) -> bool {}

    pub fn insert(&mut self, path: &Path) -> Result<(), String> {
        for root_node in self.root_nodes.iter_mut() {
            if root_node.insert(path).is_ok() {
                return Ok(());
            }
        }

        match path
            .ancestors()
            // necessary because last() yields "" as root for relative paths
            .filter(|ancestor| !ancestor.as_os_str().is_empty())
            .last() {
            None => return Err("cannot insert empty path".to_string()),
            Some(root) => {
                let mut root_node = Node {
                    path: PathBuf::from(root),
                    children: Vec::new(),
                    is_element: false,
                };
                root_node.insert(path).unwrap();

                self.root_nodes.push(root_node);

                Ok(())
            },
        }
    }

    // pub fn remove(&self, path: &Path) -> Result<(), String> {}

    pub fn iter(&self) -> PathTrieIterator {
        PathTrieIterator::new(self)
    }
}

impl Default for PathTrie {
    fn default() -> Self {
        Self::new()
    }
}
