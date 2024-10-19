use std::cmp::max;

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
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
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

    // Exercise #1: Check if the binary tree is a Binary Search Tree (BST)
    pub fn is_bst(&self) -> bool {
        self.is_bst_rec(Some(0), None, None)
    }

    pub fn is_bst_rec(&self, node_id: Option<usize>, min: Option<u32>, max: Option<u32>) -> bool {
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

    // Exercise #2: Return the sum of the maximum simple path connecting two leaves
    pub fn max_path_sum(&self) -> u32 {
        let mut max_sum = u32::MIN;
        self.max_path_sum_rec(Some(0), &mut max_sum);
        max_sum
    }

    fn max_path_sum_rec(&self, node_id: Option<usize>, max_sum: &mut u32) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            // If it's a leaf node, return its key
            if node.id_left.is_none() && node.id_right.is_none() {
                return node.key;
            }

            // Calculate the sum of the left subtree (if it exists)
            let left_sum = if let Some(left_id) = node.id_left {
                self.max_path_sum_rec(Some(left_id), max_sum)
            } else {
                0
            };

            // Calculate the sum of the right subtree (if it exists)
            let right_sum = if let Some(right_id) = node.id_right {
                self.max_path_sum_rec(Some(right_id), max_sum)
            } else {
                0
            };

            // If both children exist, update the max_sum
            if node.id_left.is_some() && node.id_right.is_some() {
                let current_sum = left_sum + node.key + right_sum;
                if current_sum > *max_sum {
                    *max_sum = current_sum;
                }
            }

            // Return the maximum path sum of either the left or right subtree (+ current node key)
            return node.key + max(left_sum, right_sum);
        }

        0
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
        // Tree:
        //        10
        //      /    \
        //     5      15
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 15, false); // id 2

        assert!(tree.is_bst()); // Should be true

        // Add more nodes but keep the tree a BST
        // Tree:
        //        10
        //      /    \
        //     5      15
        //    / \    /  \
        //   2   7  12  20
        tree.add_node(1, 2, true); // id 3
        tree.add_node(1, 7, false); // id 4
        tree.add_node(2, 12, true); // id 5
        tree.add_node(2, 20, false); // id 6

        assert!(tree.is_bst()); // Should be true

        // Add a new node that makes the tree not BST
        // Tree:
        //        10
        //      /    \
        //     5      15
        //    / \    /  \
        //   2   7  12  20
        //             /
        //            6
        tree.add_node(3, 6, true); // id 7

        assert!(!tree.is_bst()); // Should be false
    }

    #[test]
    fn test_max_path_sum() {
        // Test case: tree with only two nodes //
        // Tree:
        //     12
        let tree = Tree::with_root(12);

        assert_eq!(tree.max_path_sum(), u32::MIN); // MIN because there is no leaf-to-leaf path

        // Test case: tree with only two nodes //
        // Tree:
        //     10
        //    /
        //   5
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);

        assert_eq!(tree.max_path_sum(), u32::MIN); // MIN because there is no leaf-to-leaf path

        // Test case: normal balanced tree //
        // Tree:
        //        10
        //      /    \
        //     5      15
        //    / \    /  \
        //   3   7  12  20
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(0, 15, false);
        tree.add_node(1, 3, true);
        tree.add_node(1, 7, false);
        tree.add_node(2, 12, true);
        tree.add_node(2, 20, false);

        assert_eq!(tree.max_path_sum(), 57); // Maximum path is 7 -> 5 -> 10 -> 15 -> 20

        // Test case: more complex tree //
        // Tree:
        //        10
        //      /    \
        //     5      15
        //    / \    /  \
        //   3   7  12  20
        //  /             \
        // 1               25
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 15, false); // id 2
        tree.add_node(1, 3, true); // id 3
        tree.add_node(3, 1, true); // id 4
        tree.add_node(1, 7, false); // id 5
        tree.add_node(2, 12, true); // id 6
        tree.add_node(2, 20, false); // id 7
        tree.add_node(7, 25, false); // id 8

        assert_eq!(tree.max_path_sum(), 82); // Maximum path is 7 -> 5 -> 10 -> 15 -> 20 -> 25

        // Test case: Left-heavy tree //
        // Tree:
        //        1
        //      /
        //     2
        //    /
        //   3
        //  /
        // 4
        let mut tree = Tree::with_root(1);
        tree.add_node(0, 2, true); // id 1
        tree.add_node(1, 3, true); // id 2
        tree.add_node(2, 4, true); // id 3

        assert_eq!(tree.max_path_sum(), 0); // No path between two leaves

        // Test case 4: Right-heavy tree
        // Tree:
        //        6
        //         \
        //           3
        //            \
        //             4
        //              \
        //               5
        let mut tree = Tree::with_root(6);
        tree.add_node(0, 3, false);
        tree.add_node(1, 4, false);
        tree.add_node(2, 5, false);

        assert_eq!(tree.max_path_sum(), 0); // No path between two leaves
    }
}
