// 222. Count Complete Tree Nodes
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let mut left_height = 0;
                let mut current = node.clone();
                while let Some(left) = current.borrow().left.clone() {
                    left_height += 1;
                    current = left;
                }
                
                let mut right_height = 0;
                current = node.clone();
                while let Some(right) = current.borrow().right.clone() {
                    right_height += 1;
                    current = right;
                }
                
                if left_height == right_height {
                    return (1 << (left_height + 1)) - 1;
                }
                
                1 + Self::count_nodes(node.borrow().left.clone()) + 
                    Self::count_nodes(node.borrow().right.clone())
            }
        }
    }
}