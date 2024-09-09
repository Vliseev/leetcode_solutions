# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def spiralMatrix(self, m: int, n: int, head: Optional[ListNode]) -> List[List[int]]:
        matrix = [[-1 for _ in range(n)] for _ in range(m)]

        left, right, top, bottom = 0, n - 1, 0, m - 1

        while head:
            for j in range(left, right + 1):
                if head:
                    matrix[top][j] = head.val
                    head = head.next
                else:
                    return matrix
            top += 1
            for i in range(top, bottom + 1):
                if head:
                    matrix[i][right] = head.val
                    head = head.next
                else:
                    return matrix
            right -= 1
            for j in range(right, left - 1, -1):
                if head:
                    matrix[bottom][j] = head.val
                    head = head.next
                else:
                    return matrix
            bottom -= 1
            for i in range(bottom, top - 1, -1):
                if head:
                    matrix[i][left] = head.val
                    head = head.next
                else:
                    return matrix
            left += 1
        return matrix
