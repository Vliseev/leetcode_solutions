# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        cur_node = root
        while cur_node:
            if cur_node.val > p.val and cur_node.val > q.val:
                cur_node = cur_node.left
            elif cur_node.val < p.val and cur_node.val < q.val:
                cur_node = cur_node.right
            else:
                return cur_node
