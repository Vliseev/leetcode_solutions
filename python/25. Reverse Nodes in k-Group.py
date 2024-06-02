from typing import Optional


# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        def reverseLinkedList(head: Optional[ListNode], k: int):
            ptr = head
            prev = None
            cnt = 0

            while ptr and cnt < k:
                next = ptr.next
                ptr.next = prev
                prev, ptr = ptr, next
                cnt += 1
            return prev

        def helper(head: Optional[ListNode], k: int) -> Optional[ListNode]:
            ptr = head
            cnt = 0

            while cnt < k and ptr:
                cnt += 1
                ptr = ptr.next

            if cnt == k:
                rev_head = reverseLinkedList(head, k)
                head.next = helper(ptr, k)
                return rev_head
            return head
        
        return helper(head, k)


# Helper function to create a linked list from a list of values
def create_linked_list(lst):
    dummy = ListNode(0)
    curr = dummy
    for val in lst:
        curr.next = ListNode(val)
        curr = curr.next
    return dummy.next


# Helper function to print a linked list
def print_linked_list(head):
    while head:
        print(head.val, end=" -> ")
        head = head.next
    print("None")


lst = [1, 2, 3, 4, 5]
k = 3
head = create_linked_list(lst)

# Reversing nodes in k groups
new_head = Solution().reverseKGroup(head, 2)

# Printing the modified linked list
print_linked_list(new_head)  #
