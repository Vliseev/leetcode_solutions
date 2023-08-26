from typing import Optional


class Node:
    def __init__(
        self,
        val: int = 0,
        left: "Optional[Node]" = None,
        right: "Optional[Node]" = None,
        next: "Optional[Node]" = None,
    ):
        self.val = val
        self.left = left
        self.right = right
        self.next = next


class Solution:
    def connect(self, root: "Optional[Node]") -> "Optional[Node]":
        if root is None:
            return root
        cur, next_ptr = root, root.left
        while cur and next_ptr:
            cur.left.next = cur.right
            if cur.next:
                cur.right.next = cur.next.left
            cur = cur.next
            if not cur:
                cur = next_ptr
                next_ptr = cur.left
        return root
