class Solution {
public:
    int pairSum(ListNode* head) {
        // Find middle (slow/fast pointers)
        ListNode* slow = head;
        ListNode* fast = head;
        while (fast && fast->next) {
            slow = slow->next;
            fast = fast->next->next;
        }
        
        // Reverse second half
        ListNode* prev = nullptr;
        while (slow) {
            ListNode* next = slow->next;
            slow->next = prev;
            prev = slow;
            slow = next;
        }
        
        // Calculate max twin sum
        int maxSum = 0;
        ListNode* first = head;
        ListNode* second = prev;
        while (second) {
            maxSum = max(maxSum, first->val + second->val);
            first = first->next;
            second = second->next;
        }
        
        return maxSum;
    }
};