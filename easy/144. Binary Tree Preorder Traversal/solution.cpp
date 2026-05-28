/*
Time: O(n)
Space: O(n)

Idea:
Preorder traversal order:
1. current node
2. left subtree
3. right subtree

Use simple DFS recursion.
*/

class Solution {
public:
    vector<int> answer;

    void dfs(TreeNode* node) {
        if (node == nullptr) {
            return;
        }

        // visit current node first
        answer.push_back(node->val);

        // then go left
        dfs(node->left);

        // then go right
        dfs(node->right);
    }

    vector<int> preorderTraversal(TreeNode* root) {
        dfs(root);

        return answer;
    }
};