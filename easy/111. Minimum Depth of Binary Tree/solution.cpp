/*
Time: O(n)
Space: O(h)

Idea: DFS, ignore missing child when taking minimum depth
*/

class Solution {
public:
    int minDepth(TreeNode* root) {
        if (!root) return 0;

        int left = minDepth(root->left);
        int right = minDepth(root->right);

        // if one child missing, take the other path
        if (!left || !right) {
            return 1 + left + right;
        }

        return 1 + min(left, right);
    }
};