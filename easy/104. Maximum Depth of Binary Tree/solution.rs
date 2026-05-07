/*
Time: O(n)
Space: O(h)

Idea: depth = 1 + max(left depth, right depth)
*/

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                1 + std::cmp::max(
                    Self::max_depth(node.borrow().left.clone()),
                    Self::max_depth(node.borrow().right.clone())
                )
            }
        }
    }
}