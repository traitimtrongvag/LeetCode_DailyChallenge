/*
Time: O(n)
Space: O(1)
Beats: 100%
Idea: count length, connect tail to head (cycle), move to (n - k % n), cut there
*/

class Solution {
public:
    ListNode* rotateRight(ListNode* head, int k) {
        if (!head || !head->next || k == 0) return head;

        // count length
        int n = 1;
        ListNode* tail = head;
        while (tail->next) {
            tail = tail->next;
            n++;
        }

        k %= n;
        if (k == 0) return head;

        // make circular
        tail->next = head;

        // find new tail
        int steps = n - k;
        ListNode* cur = head;
        for (int i = 1; i < steps; i++) {
            cur = cur->next;
        }

        // break circle
        ListNode* newHead = cur->next;
        cur->next = nullptr;

        return newHead;
    }
};