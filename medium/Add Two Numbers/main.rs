// 2. Add Two Numbers
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let val1 = l1.as_ref().map_or(0, |n| n.val);
            let val2 = l2.as_ref().map_or(0, |n| n.val);
            let total = val1 + val2 + carry;
            carry = total / 10;
            current.next = Some(Box::new(ListNode::new(total % 10)));
            current = current.next.as_mut().unwrap();
            if let Some(node) = l1 { l1 = node.next; } else { l1 = None; }
            if let Some(node) = l2 { l2 = node.next; } else { l2 = None; }
        }

        dummy.next
    }
}