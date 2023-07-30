# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        self.cnt = 0
        self.result = None

        def rec(cur_node: Optional[TreeNode]):
            if cur_node is None:
                return cur_node
            val = rec(cur_node.left)
            if not val is None:
                return val
            self.cnt += 1
            if self.cnt == k:
                return cur_node.val
            return rec(cur_node.right)

        return rec(root)

