# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def widthOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        prev_level, prev_num = 0, 0
        queue = deque([(root, 0, 0)])
        
        result = 1

        while queue:
            node, node_level, node_idx = queue.popleft()
            if prev_level != node_level:
                prev_level = node_level
                prev_num = node_idx
            result = max(result, node_idx - prev_num + 1)
            if node.left:
                queue.append((node.left, node_level + 1, 2 * node_idx + 1))
            if node.right:
                queue.append((node.right, node_level + 1, 2 * node_idx + 2))
        
        return result
