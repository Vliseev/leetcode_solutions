# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

def gcd(a, b):
    """Calculate the greatest common divisor of two numbers."""
    if b == 0:
        return a
    return gcd(b, a % b)

class Solution:
    def insertGreatestCommonDivisors(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None or head.next is None:
            return head
        cur = head

        while cur.next:
            a = cur
            b = cur.next
            gcd_val = gcd(a.val, b.val)
            gcd_node = ListNode(gcd_val)

            tmp = a.next
            cur.next = gcd_node
            gcd_node.next = tmp
            cur = tmp
        return head
