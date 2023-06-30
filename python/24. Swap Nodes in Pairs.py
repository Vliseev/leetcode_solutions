# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def swapPairs(self, head: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode(0, head)
        prev, cur = dummy, head
        while cur and cur.next:
            ptNext = cur.next.next
            second = cur.next

            prev.next = second
            second.next = cur

            cur.next = ptNext

            prev = cur
            cur = cur.next
        return dummy.next
