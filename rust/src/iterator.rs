use crate::{
    node::{
        iterator::NodeIterator,
        Node,
    },
    PathTrie,
};
use std::{
    path::PathBuf,
    slice::Iter,
};


pub struct PathTrieIterator<'a> {
    root_node_iterator: Iter<'a, Node>,
    root_node: Option<&'a Node>,
    node_iterator: Option<NodeIterator<'a>>,
}

impl<'a> PathTrieIterator<'a> {
    pub(crate) fn new(path_trie: &'a PathTrie) -> Self {
        Self {
            root_node_iterator: path_trie.root_nodes.iter(),
            root_node: None,
            node_iterator: None,
        }
    }
}

impl<'a> Iterator for PathTrieIterator<'a> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        if self.root_node.is_none() {
            self.root_node = self.root_node_iterator.next();
            if self.root_node.is_none() {
                return None;
            }
        }

        while let Some(root_node) = self.root_node {
            if self.node_iterator.is_none() {
                self.node_iterator = Some(root_node.iter());
            }
            let node_iterator = self.node_iterator.as_mut().unwrap();

            match node_iterator.next() {
                None => self.root_node = self.root_node_iterator.next(),
                Some(path) => return Some(path),
            }
        }

        None
    }
}
