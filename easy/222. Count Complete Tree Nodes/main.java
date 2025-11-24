// 222. Count Complete Tree Nodes
class Solution {
    public int countNodes(TreeNode root) {
        if (root == null) return 0;
        int leftHeight = 0, rightHeight = 0;
        TreeNode left = root;
        TreeNode right = root;
        
        while (left != null) {
            leftHeight++;
            left = left.left;
        }
        while (right != null) {
            rightHeight++;
            right = right.right;
        }
        
        if (leftHeight == rightHeight) {
            return (1 << leftHeight) - 1;
        }
        
        return 1 + countNodes(root.left) + countNodes(root.right);
    }
}