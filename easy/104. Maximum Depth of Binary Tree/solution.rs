use std::rc::Rc;
use std::cell::RefCell;

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