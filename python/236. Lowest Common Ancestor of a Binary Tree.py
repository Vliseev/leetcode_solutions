# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def lowestCommonAncestor(
        self, root: "TreeNode", p: "TreeNode", q: "TreeNode"
    ) -> "TreeNode":
        def helper(node):
            if node is None:
                return None
            if node.val == p.val or node.val == q.val:
                return node
            
            left = helper(node.left)
            right = helper(node.right)
            if left is None:
                return right
            elif right is None:
                return left
            else:
                return node
        return helper(root)

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def lowestCommonAncestor(
        self, root: "TreeNode", p: "TreeNode", q: "TreeNode"
    ) -> "TreeNode":
        def find_path(root, target):
            def helper(node, path):
                if not node:
                    return None
                path.append(node)

                if node.val == target:
                    return path
                left_path = helper(node.left, path)
                if left_path:
                    return left_path
                right_path = helper(node.right, path)
                if right_path:
                    return right_path
                path.pop()

                return None
            return helper(root, [])

        p_path = find_path(root, p.val)
        q_path = find_path(root, q.val)

        if p_path is None or q_path is None:
            return None

        min_len = min(len(p_path), len(q_path))
        for i in range(min_len):
            if p_path[i].val != q_path[i].val:
                return p_path[i-1]

        return p_path[min_len-1]
