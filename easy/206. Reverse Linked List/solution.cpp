/*
Time: O(n)
Space: O(1)

Idea: reverse pointers one by one
*/

class Solution {
public:
    ListNode* reverseList(ListNode* head) {
        ListNode* prev = nullptr;
        ListNode* cur = head;

        while (cur) {
            ListNode* next = cur->next;

            cur->next = prev;
            prev = cur;
            cur = next;
        }

        return prev;
    }
};