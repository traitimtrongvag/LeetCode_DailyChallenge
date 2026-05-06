/*
Time: O(n)
Space: O(h)

Idea: compare nodes, then recurse left and right
*/

class Solution {
public:
    bool isSameTree(TreeNode* p, TreeNode* q) {
        if (!p && !q) return true;     // both null
        if (!p || !q) return false;    // one null
        if (p->val != q->val) return false;

        // check left and right
        return isSameTree(p->left, q->left) &&
               isSameTree(p->right, q->right);
    }
};