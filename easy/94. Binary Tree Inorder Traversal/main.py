# 94. Binary Tree Inorder Traversal
class Solution:
    def inorderTraversal(self, root):
        res, st = [], []
        cur = root
        while cur or st:
            while cur:
                st.append(cur)
                cur = cur.left
            cur = st.pop()
            res.append(cur.val)
            cur = cur.right
        return res