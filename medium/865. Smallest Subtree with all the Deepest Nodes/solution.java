class Solution {
    public TreeNode subtreeWithAllDeepest(TreeNode root) {
        if (root == null) return null;

        Map<TreeNode, TreeNode> parent = new HashMap<>();
        Queue<TreeNode> q = new LinkedList<>();
        q.offer(root);
        parent.put(root, null);

        List<TreeNode> lastLevel = new ArrayList<>();

        // BFS traversal
        while (!q.isEmpty()) {
            int size = q.size();
            lastLevel.clear();

            for (int i = 0; i < size; i++) {
                TreeNode node = q.poll();
                lastLevel.add(node);

                if (node.left != null) {
                    parent.put(node.left, node);
                    q.offer(node.left);
                }
                if (node.right != null) {
                    parent.put(node.right, node);
                    q.offer(node.right);
                }
            }
        }

        Set<TreeNode> deepest = new HashSet<>(lastLevel);

        // Move upward until all nodes converge
        while (deepest.size() > 1) {
            Set<TreeNode> next = new HashSet<>();
            for (TreeNode node : deepest) {
                next.add(parent.get(node));
            }
            deepest = next;
        }

        return deepest.iterator().next();
    }
}