class Solution:
    def removeStones(self, stones: List[List[int]]) -> int:
        def find_root(el):
            if parent[el] != el:
                parent[el] = find_root(parent[el]) #path comprehension
            return parent[el]
        num_nodes = 100000
        parent = list(range(2*num_nodes))
        for (x, y) in stones:
            parent[find_root(x)] = find_root(y+num_nodes) # parent of row - col
        return len(stones) - len(set(find_root(x) for x, _ in stones))
