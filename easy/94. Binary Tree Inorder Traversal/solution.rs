use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];
        let mut cur = root;

        while cur.is_some() || !stack.is_empty() {
            while let Some(node_rc) = cur {
                stack.push(Rc::clone(&node_rc));
                cur = node_rc.borrow().left.clone();
            }
            let node_rc = stack.pop().unwrap();
            res.push(node_rc.borrow().val);
            cur = node_rc.borrow().right.clone();
        }

        res
    }
}