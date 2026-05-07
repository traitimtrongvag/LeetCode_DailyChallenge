/*
Time: O(n)
Space: O(h)

Idea: depth = 1 + max(left depth, right depth)
*/

int maxDepth(struct TreeNode* root) {
    if (!root) return 0;

    int left = maxDepth(root->left);
    int right = maxDepth(root->right);

    return 1 + (left > right ? left : right);
}