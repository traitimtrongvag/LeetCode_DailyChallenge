use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p_node), Some(q_node)) => {
                let p_borrow = p_node.borrow();
                let q_borrow = q_node.borrow();
                p_borrow.val == q_borrow.val 
                    && Self::is_same_tree(p_borrow.left.clone(), q_borrow.left.clone())
                    && Self::is_same_tree(p_borrow.right.clone(), q_borrow.right.clone())
            }
            _ => false
        }
    }
}