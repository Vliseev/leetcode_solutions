"""
# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
"""

class Solution:
    def connect(self, root: "Optional[Node]") -> "Optional[Node]":
        if root is None:
            return root
        top_node = root
        while top_node:
            cur = top_node
            dummy = Node()
            prev = dummy
            while cur:
                if cur.left:
                    prev.next = cur.left
                    prev = prev.next
                if cur.right:
                    prev.next = cur.right
                    prev = prev.next
                cur = cur.next
            top_node = dummy.next
        return root

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

