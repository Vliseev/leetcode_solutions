from typing import (
    List,
)
from lintcode import (
    TreeNode,
)

"""
Definition of TreeNode:
class TreeNode:
    def __init__(self, val):
        self.val = val
        self.left, self.right = None, None
"""

from collections import defaultdict, deque

class Solution:
    """
    @param root: the root of tree
    @return: the vertical order traversal
    """
    def vertical_order(self, root: TreeNode) -> List[List[int]]:
        # write your code here
        col_map = defaultdict(list)

        queue = deque([(root, 0, 0)])
        while queue:
            n = len(queue)
            node, row, col = queue.popleft()
            if node is None:
                continue
            col_map[col].append((row, node.val))
            queue.append((node.left, row + 1, col - 1))
            queue.append((node.right, row + 1, col + 1))

        result = []
        for col in sorted(col_map.keys()):
            result.append([val for _, val in col_map[col]])
        return result


"""
           3 (0,0)
          /      \
   9 (1,-1)      20 (1,1)
                /         \
         15 (2,0)        7 (2,2)
"""
