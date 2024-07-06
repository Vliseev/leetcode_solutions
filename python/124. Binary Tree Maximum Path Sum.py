# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        def helper(root: Optional[TreeNode]) -> int:
            if root is None:
                return 0
            left_max = max(helper(root.left), 0)
            right_max = max(helper(root.right), 0)
            cur_val = left_max + root.val + right_max

            nonlocal ans
            ans = max(ans, cur_val)
            return root.val + max(left_max, right_max)
        ans = float("-inf")
        helper(root)
        return ans
