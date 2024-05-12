class TrieNode:
    def __init__(self) -> None:
        self.nodes = [None, None]
        self.val = 0

class Trie:
    NUM_BIT = 31
    def __init__(self):
        self.root = TrieNode()
    
    def add_number(self, number):
        cur_node = self.root
        for bit_id in range(Trie.NUM_BIT, -1, -1):
            cur_bit = (number >> bit_id) & 1
            if cur_node.nodes[cur_bit] is None:
                cur_node.nodes[cur_bit] = TrieNode()
            cur_node = cur_node.nodes[cur_bit]
        cur_node.val = number

class Solution:
    def findMaximumXOR(self, nums: List[int]) -> int:
        trie = Trie()
        max_ans = 0
        for num in nums:
            trie.add_number(num)
        for num in nums:
            node = trie.root
            for bit_id in range(Trie.NUM_BIT, -1, -1):
                cur_bit = (num >> bit_id) & 1

                need_val = 1 - cur_bit
                node = node.nodes[need_val] if node.nodes[need_val] else node.nodes[cur_bit]
            max_ans = max(max_ans, num ^ node.val)
        return max_ans


class Solution2:
    def findMaximumXOR(self, nums: List[int]) -> int:
        max_xor = 0
        mask = 0
        
        for i in range(31, -1, -1):
            mask |= (1 << i)
            prefixes = {num & mask for num in nums}
            temp = max_xor | (1 << i)
            if any(temp ^ p in prefixes for p in prefixes):
                max_xor = temp
        
        return max_xor
