pub(crate) mod iterator;


use ancestry::Ancestry;
use iterator::NodeIterator;
use std::path::{
    Path,
    PathBuf,
};


#[derive(Debug)]
pub(crate) struct Node {
    pub(crate) path: PathBuf,
    pub(crate) children: Vec<Node>,
}

impl Node {
    pub(crate) fn insert(&mut self, path: &Path) -> Result<(), String> {
        // Is path already in?
        if path == self.path {
            return Err(format!(
                "path \"{}\" already in trie",
                path.to_string_lossy(),
            ));
        }

        // Does path even belong?
        let path = match path.strip_prefix(&self.path) {
            // If so, strip for daddy.
            Ok(stripped_path) => stripped_path,
            Err(_) => return Err(format!(
                "path \"{}\" cannot be inserted under path \"{}\"",
                path.to_string_lossy(),
                self.path.to_string_lossy(),
            )),
        };

        // Iterate through children.
        let mut problem_child = None;
        for (index, child) in self.children.iter_mut().enumerate()  {
            // Does path belong under child?
            if path.starts_with(&child.path) {
                return child.insert(path);
            }

            // This is where it gets tricky. If path and child share a common
            // prefix, at the very least, we know the problem child doesn't
            // belong under its current parent.
            //
            // The borrow checker doesn't make it very natural to perform
            // complex modifications on collections while iterating through
            // them, so mark it for after the loop.
            if let Some(common_prefix) =
                child.path.closest_common_ancestor(path) {
                problem_child = Some((index, common_prefix.to_path_buf()));
                break;
            }
        }

        // Is there a problem child?
        if let Some((problem_child_index, common_prefix)) = problem_child {
            // Remove the problem child from its current parent and strip for
            // daddy.
            let mut problem_child = self.children.remove(problem_child_index);
            problem_child.path = problem_child
                .path
                .strip_prefix(&common_prefix)
                .unwrap()
                .to_path_buf();

            // Handle the case where path is itself a prefix of child.
            if common_prefix == path {
                self.children.push(Node {
                    path: path.to_path_buf(),
                    children: vec![problem_child],
                });

                return Ok(());
            }

            // Create a new prefix node containing path and child.
            self.children.push(Node {
                path: common_prefix.to_path_buf(),
                children: vec![
                    Node {
                        path: path
                            .strip_prefix(common_prefix)
                            .unwrap()
                            .to_path_buf(),
                        children: Vec::new(),
                    },
                    problem_child,
                ],
            });

            return Ok(());
        }

        // Finally, here's the easy case. Just add the path to the current
        // node's children.
        self.children.push(Node {
            path: path.to_path_buf(),
            children: Vec::new(),
        });

        Ok(())
    }

    pub fn iter(&self) -> NodeIterator {
        NodeIterator::new(self)
    }
}
