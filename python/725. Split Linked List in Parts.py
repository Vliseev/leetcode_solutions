# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def splitListToParts(self, head: Optional[ListNode], k: int) -> List[Optional[ListNode]]:
        size = 0
        cur = head
        while cur:
            size += 1
            cur = cur.next

        result = []
        batch_size, remainder = divmod(size, k)
        cur = head
        for i in range(k):
            result.append(cur)
            cur_size = batch_size + (1 if i < remainder else 0)
            for j in range(cur_size - 1):
                if cur:
                    cur = cur.next
            if cur:
                tmp = cur.next
                cur.next, cur = None, tmp
        return result
