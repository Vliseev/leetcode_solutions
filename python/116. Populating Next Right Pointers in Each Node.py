"""
# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
"""

# class Solution:
#     def connect(self, root: 'Optional[Node]') -> 'Optional[Node]':
#         if root is None:
#             return root
#         cur, next_ptr = root, root.left
#         while cur and next_ptr:
#             cur.left.next = cur.right
#             if cur.next:
#                 cur.right.next = cur.next.left
#             cur = cur.next
#             if not cur:
#                 cur = next_ptr
#                 next_ptr = cur.left
#         return root


from collections import deque


class Solution:
    def connect(self, root: "Optional[Node]") -> "Optional[Node]":
        if root is None:
            return root
        queue = deque([root])

        while queue:
            n = len(queue)
            for i in range(n):
                el = queue.popleft()
                if i == n - 1:
                    el.next = None
                else:
                    el.next = queue[0]
                if el.left:
                    queue.append(el.left)
                if el.right:
                    queue.append(el.right)
        return root