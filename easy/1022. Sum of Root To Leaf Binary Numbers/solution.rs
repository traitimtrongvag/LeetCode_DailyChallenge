// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sum_root_to_leaf(
        root: Option<Rc<RefCell<TreeNode>>>
    ) -> i32 {
        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            current: i32
        ) -> i32 {
            if let Some(n) = node {
                let n_ref = n.borrow();

                // Shift left (multiply by 2) and add current bit
                let value = (current << 1) | n_ref.val;

                // If leaf node, return the binary number formed
                if n_ref.left.is_none() && n_ref.right.is_none() {
                    return value;
                }

                // Recurse left and right
                dfs(n_ref.left.clone(), value)
                    + dfs(n_ref.right.clone(), value)
            } else {
                0
            }
        }

        dfs(root, 0)
    }
}