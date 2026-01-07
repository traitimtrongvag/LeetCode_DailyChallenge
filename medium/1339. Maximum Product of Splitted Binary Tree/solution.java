/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
 import java.io.PrintStream;

class Solution {
    private long total, output;
    private long mod = 1000000007;

    private long dfs(TreeNode root) {
        if (root == null) return 0;

        long left = dfs(root.left);
        long right = dfs(root.right);

        long outputleft = (total - left) * left;
        long outputright = (total - right) * right;

        output = Math.max(output, outputleft);
        output = Math.max(output, outputright);

        return (left + right + root.val);
    }

    private void sum(TreeNode root) {
        if (root == null) return;

        total = (total + root.val);
        sum(root.left);
        sum(root.right);
        return;
    }

    public int maxProduct(TreeNode root) {
        total = 0;
        output = Long.MIN_VALUE;

        PrintStream ps = System.out;

        sum(root);
        ps.println(total);
        dfs(root);

        return (int)(output % mod);
    }
}