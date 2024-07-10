# Definition for a binary tree node.
# class TreeNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Codec:
    def serialize(self, root: TreeNode):
        def dfs(root):
            if root is None:
                return "#"
            return ",".join((str(root.val), dfs(root.left), dfs(root.right)))

        return dfs(root)

    def deserialize(self, data: str):
        """Decodes your encoded data to tree.

        :type data: str
        :rtype: TreeNode
        """
        data_it = iter(data.split(','))

        def dfs(data_it):
            node_val = next(data_it)
            if node_val == "#":
                return None
            node = TreeNode(node_val)
            node.left = dfs(data_it)
            node.right = dfs(data_it)
            return node
        return dfs(data_it)
        

# Your Codec object will be instantiated and called as such:
# ser = Codec()
# deser = Codec()
# ans = deser.deserialize(ser.serialize(root))
