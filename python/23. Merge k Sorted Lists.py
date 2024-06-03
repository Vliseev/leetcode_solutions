# Definition for singly-linked list.
from typing import List, Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        def merge_two_list(lst1: ListNode, lst2: ListNode) -> ListNode:
            dummy = ListNode(0)
            cur = dummy
            while lst1 and lst2:
                if lst1.val < lst2.val:
                    cur.next = lst1
                    lst1 = lst1.next
                else:
                    cur.next = lst2
                    lst2 = lst2.next
                cur = cur.next
            cur.next = lst1 if lst1 else lst2
            return dummy.next

        def rec_foo(lists: List[Optional[ListNode]]) -> Optional[ListNode]:
            if not lists:
                return None
            n = len(lists)

            if n == 1:
                return lists[0]

            mid = n // 2
            lst1 = rec_foo(lists[:mid])
            lst2 = rec_foo(lists[mid:])
            return merge_two_list(lst1, lst2)

        return rec_foo(lists)

def create_linked_list(arr):
    dummy = ListNode(0)
    current = dummy
    for value in arr:
        current.next = ListNode(value)
        current = current.next
    return dummy.next

def print_linked_list(node):
    while node:
        print(node.val, end=' -> ')
        node = node.next
    print('None')

lists = [
    create_linked_list([1, 4, 5]),
    create_linked_list([1, 3, 4]),
    create_linked_list([2, 6])
]

merged_list = Solution().mergeKLists(lists)
print_linked_list(merged_list)
