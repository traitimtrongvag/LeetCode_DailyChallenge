// 23. Merge k Sorted Lists
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Reverse<Box<ListNode>>> = BinaryHeap::new();

        for node in lists {
            if let Some(n) = node {
                heap.push(Reverse(n));
            }
        }

        let mut dummy = Box::new(ListNode { val: 0, next: None });
        let mut tail = &mut dummy;

        while let Some(Reverse(mut node)) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(Reverse(next));
            }
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}