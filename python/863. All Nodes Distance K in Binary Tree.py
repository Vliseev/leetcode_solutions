# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def distanceK(self, root: TreeNode, target: TreeNode, k: int) -> List[int]:
        map_nodes = {}
        queue = deque([root])

        target_node = None
        while queue:
            cur = queue.popleft()

            if cur.val == target.val:
                target_node = cur
                break
            if cur.left:
                map_nodes[cur.left.val] = cur
                queue.append(cur.left)
            if cur.right:
                map_nodes[cur.right.val] = cur
                queue.append(cur.right)

        dist = 0
        queue = deque([target])
        visited = set()
        while dist < k:
            n = len(queue)
            for _ in range(n):
                el = queue.popleft()
                visited.add(el.val)
                if el.left and not el.left.val in visited:
                    queue.append(el.left)
                if el.right and not el.right.val in visited:
                    queue.append(el.right)
                if el.val in map_nodes and not map_nodes[el.val].val in visited:
                    queue.append(map_nodes[el.val])
            dist += 1
        return [el.val for el in queue]
