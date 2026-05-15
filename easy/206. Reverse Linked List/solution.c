/*
Time: O(n)
Space: O(1)

Idea: reverse pointers one by one
*/

struct ListNode* reverseList(struct ListNode* head) {
    struct ListNode* prev = NULL;
    struct ListNode* cur = head;

    while (cur) {
        struct ListNode* next = cur->next;

        cur->next = prev;
        prev = cur;
        cur = next;
    }

    return prev;
}