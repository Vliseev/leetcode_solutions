# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseBetween(self, head: Optional[ListNode], left: int, right: int) -> Optional[ListNode]:
        dummy = ListNode(next=head)
        cur = head
        prev, next_ptr = dummy, None
        for _ in range(left-1):
            prev, cur = cur, cur.next
        first = prev
        second = cur
        for _ in range(right-left+1):
            next_ptr = cur.next
            cur.next = prev
            prev = cur
            cur = next_ptr
        first.next = prev
        second.next = cur

        return dummy.next
