#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        Self::insert_node(&mut self.root, value);
    }

    fn insert_node(slot: &mut Option<Box<Node<T>>>, value: T) {
        match slot {
            None => {
                *slot = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }));
            }
            Some(node) => {
                if value <= node.value {
                    Self::insert_node(&mut node.left, value);
                } else {
                    Self::insert_node(&mut node.right, value);
                }
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        let mut curr = &self.root;
        while let Some(node) = curr {
            if value == &node.value {
                return true;
            } else if value < &node.value {
                curr = &node.left;
            } else {
                curr = &node.right;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let mut tree = BinaryTree::new();
        tree.insert(50);
        tree.insert(30);
        tree.insert(70);

        assert!(tree.contains(&30));
        assert!(tree.contains(&70));
        assert!(!tree.contains(&100));
    }
}
