use crate::node::Node;
use std::path::PathBuf;


pub(crate) struct NodeIterator<'a> {
    node: &'a Node,
    node_path_yielded: bool,
    child_index: usize,
    child_iterator: Option<Box<NodeIterator<'a>>>,
}

impl<'a> NodeIterator<'a> {
    pub(crate) fn new(node: &'a Node) -> Self {
        Self {
            node,
            node_path_yielded: false,
            child_index: 0,
            child_iterator: None,
        }
    }
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        if self.node.is_element && !self.node_path_yielded {
            self.node_path_yielded = true;
            return Some(self.node.path.clone());
        }

        while self.child_index < self.node.children.len() {
            if self.child_iterator.is_none() {
                self.child_iterator = Some(Box::new(self
                    .node
                    .children
                    .get(self.child_index)
                    .unwrap()
                    .iter()
                ));
            }

            if let Some(next_item) = self
                .child_iterator
                .as_mut()
                .unwrap()
                .next() {
                let mut path = self.node.path.clone();
                path.push(next_item);
                println!("yielding child next: \"{}\"", path.to_string_lossy());
                return Some(path);
            }

            self.child_iterator = None;
            self.child_index += 1;
        }

        None
    }
}
