# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        queue = deque([root])
        result = []

        while queue:
            rightMost = None
            n = len(queue)
            for _ in range(n):
                node = queue.popleft()
                if node:
                    rightMost = node
                    queue.append(node.left)
                    queue.append(node.right)
            if rightMost:
                result.append(rightMost.val)
        return result
