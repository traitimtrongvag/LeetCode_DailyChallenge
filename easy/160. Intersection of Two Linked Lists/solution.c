struct ListNode *getIntersectionNode(struct ListNode *headA, struct ListNode *headB) {
    struct ListNode *a = headA;
    struct ListNode *b = headB;
    
    while (a != b) {
        a = a ? a->next : headB;
        b = b ? b->next : headA;
    }
    
    return a;
}