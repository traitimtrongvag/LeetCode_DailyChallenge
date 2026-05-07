/*
Time: O(n)
Space: O(h)

Idea: depth = 1 + max(left depth, right depth)
*/

class Solution {
public:
    int maxDepth(TreeNode* root) {
        if (!root) return 0;

        return 1 + max(
            maxDepth(root->left),
            maxDepth(root->right)
        );
    }
};