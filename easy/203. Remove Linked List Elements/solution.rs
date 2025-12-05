impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut curr = &mut dummy;
        
        while let Some(ref mut next_node) = curr.next {
            if next_node.val == val {
                curr.next = next_node.next.take();
            } else {
                curr = curr.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}