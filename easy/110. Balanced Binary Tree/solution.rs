// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut max_diff = 0;

        fn dfs(max_diff: &mut i32, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if root.is_none() {
                // return height
                return -1;
            }

            let left_h = dfs(max_diff, root.clone().unwrap().borrow().left.clone());
            let right_h = dfs(max_diff, root.unwrap().borrow().right.clone());

            *max_diff = std::cmp::max(*max_diff, i32::abs(right_h - left_h));

            // return the height for this Node
            1 + std::cmp::max(left_h, right_h)
        }

        dfs(&mut max_diff, root);

        max_diff < 2

    }
}