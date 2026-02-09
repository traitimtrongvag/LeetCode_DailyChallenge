use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // Step 1: Perform inorder traversal to get sorted nodes
        let mut nodes = Vec::new();
        Self::inorder(root, &mut nodes);

        // Step 2: Rebuild a balanced BST from sorted nodes
        Self::construct(&nodes)
    }

    // Collect nodes in sorted (inorder) order
    fn inorder(node: Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<Rc<RefCell<TreeNode>>>) {
        if let Some(n) = node {
            // Temporarily take children to avoid borrowing issues
            let left = n.borrow_mut().left.take();
            let right = n.borrow_mut().right.take();

            Self::inorder(left, nodes);
            nodes.push(n);
            Self::inorder(right, nodes);
        }
    }

    // Recursively build a balanced BST by choosing middle element as root
    fn construct(nodes: &[Rc<RefCell<TreeNode>>]) -> Option<Rc<RefCell<TreeNode>>> {
        if nodes.is_empty() {
            return None;
        }

        // Choose middle element to ensure balance
        let mid = nodes.len() / 2;
        let root = Rc::clone(&nodes[mid]);

        {
            let mut r = root.borrow_mut();
            // Recursively build left subtree from left half
            r.left = Self::construct(&nodes[..mid]);
            // Recursively build right subtree from right half
            r.right = Self::construct(&nodes[mid + 1..]);
        }

        Some(root)
    }
}