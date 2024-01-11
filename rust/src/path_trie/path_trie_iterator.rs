use crate::{
    node::node_iterator::NodeIterator,
    PathTrie,
};
use std::path::PathBuf;


pub struct PathTrieIterator<'a> {
    root_node_iterator: NodeIterator<'a>,
}

impl<'a> PathTrieIterator<'a> {
    pub(crate) fn new(path_trie: &'a PathTrie) -> Self {
        Self {
            root_node_iterator: path_trie.root_node.iter(),
        }
    }
}

impl<'a> Iterator for PathTrieIterator<'a> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        self.root_node_iterator.next()
    }
}
