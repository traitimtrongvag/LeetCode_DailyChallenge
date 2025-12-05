type ListPtr = Box<ListNode>;
type ListOpt = Option<ListPtr>;

impl Solution {
    pub fn delete_duplicates(mut head: ListOpt) -> ListOpt {
        let mut curr_opt = head.as_mut();
        
        while let Some(curr) = curr_opt {
            let mut next_opt = curr.next.take();
            
            while let Some(next) = next_opt.as_mut() {    
                if next.val == curr.val { 
                    next_opt  = next.next.take(); 
                } else { 
                    curr.next = next_opt;  
                    break; 
                }
            }
            curr_opt = curr.next.as_mut();
        }
        head
    }
}