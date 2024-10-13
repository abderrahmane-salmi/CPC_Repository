pub struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left == None,
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right == None,
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    // TODO: Just for testing purposes
    pub fn inorder(&self) {
        self.inorder_rec(Some(0));
    }

    pub fn inorder_rec(&self, node_id: Option<usize>) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            self.inorder_rec(node.id_left);
            print!("{} ", node.key);
            self.inorder_rec(node.id_right);
        }

        return;
    }

    // Sub tree size
    pub fn subtree_size(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sl = self.subtree_size(node.id_left);
            let sr = self.subtree_size(node.id_right);

            let su = sl + sr + 1;
            return su;
        }

        0
    }

    // Exercise #1: Check if the binary tree is a Binary Search Tree (BST)
    pub fn is_bst(&self) -> bool {
        self.is_bst_rec(Some(0), None, None)
    }

    fn is_bst_rec(&self, node_id: Option<usize>, min: Option<u32>, max: Option<u32>) -> bool {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            // Check if the current node satisfies the BST properties
            if let Some(min_val) = min {
                if node.key <= min_val {
                    return false;
                }
            }
            if let Some(max_val) = max {
                if node.key >= max_val {
                    return false;
                }
            }

            // Check if the left and right subtrees are BST
            let is_left_bst = self.is_bst_rec(node.id_left, min, Some(node.key));
            let is_right_bst = self.is_bst_rec(node.id_right, Some(node.key), max);

            is_left_bst && is_right_bst
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.sum(), 64);
    }

    #[test]
    fn test_is_bst() {
        let mut tree = Tree::with_root(10);
        
        tree.add_node(0, 5, true);   // id 1
        tree.add_node(0, 15, false); // id 2
        
        assert!(tree.is_bst()); // Should be true

        // Add more nodes but keep the tree a BST
        tree.add_node(1, 2, true);  // id 3
        tree.add_node(1, 7, false); // id 4
        tree.add_node(2, 12, true); // id 5
        tree.add_node(2, 20, false); // id 6
        
        assert!(tree.is_bst()); // Should be true

        // Add a new node that makes the tree not BST
        tree.add_node(3, 6, true); // id 7
        
        assert!(!tree.is_bst()); // Should be false
    }
}

fn main() {
    println!("start main:");

    let mut tree = Tree::with_root(15);
    tree.add_node(0, 8, true); // id 1
    tree.add_node(0, 20, false); // id 2

    tree.add_node(1, 3, true); // id 3
    tree.add_node(1, 9, false); // id 4

    tree.inorder();
    println!("\n");
    println!("{}", tree.subtree_size(Some(1)));

    println!("{}", tree.is_bst());
}
