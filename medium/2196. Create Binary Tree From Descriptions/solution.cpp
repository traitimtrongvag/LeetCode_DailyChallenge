/*
Time: O(n)
Space: O(n)

Idea:
Create all nodes first.

For each description:
- connect parent -> child
- mark child as having a parent

The root is the only node that never appears as a child.
*/

class Solution {
public:
    TreeNode* createBinaryTree(vector<vector<int>>& descriptions) {
        unordered_map<int, TreeNode*> nodes;
        unordered_set<int> children;

        for (auto& d : descriptions) {
            int parent = d[0];
            int child = d[1];
            int isLeft = d[2];

            if (nodes.count(parent) == 0) {
                nodes[parent] = new TreeNode(parent);
            }

            if (nodes.count(child) == 0) {
                nodes[child] = new TreeNode(child);
            }

            if (isLeft) {
                nodes[parent]->left = nodes[child];
            } else {
                nodes[parent]->right = nodes[child];
            }

            children.insert(child);
        }

        // root never appears as a child
        for (auto& [value, node] : nodes) {
            if (children.count(value) == 0) {
                return node;
            }
        }

        return nullptr;
    }
};