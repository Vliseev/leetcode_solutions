class SegmentTree:
    def __init__(self, size):
        self.size = size
        self.tree = [0] * (2 * size)

    def update(self, index, value):
        index += self.size
        self.tree[index] += value
        while index > 1:
            index //= 2
            self.tree[index] = self.tree[2 * index] + self.tree[2 * index + 1]

    def query(self, left, right):
        left += self.size
        right += self.size
        sum = 0
        while left < right:
            if left % 2 == 1:
                sum += self.tree[left]
                left += 1
            if right % 2 == 1:
                right -= 1
                sum += self.tree[right]
            left //= 2
            right //= 2
        return sum
     """
              [1, 8]
             /        \
        [1, 4]        [5, 8]
        /  \        /  \
      [1, 2] [3, 4] [5, 6] [7, 8]
        / \   / \   / \   / \
       1   2 3   4 5   6 7   8

     If left % 2 == 1, then we get the right child and need to track the value (parent contains summ).
     If right % 2 == 1 then we get left child (the right border is not included)

    """

class Solution:
    def countRangeSum(self, nums: List[int], lower: int, upper: int) -> int:
        if not nums:
            return 0
        
        prefix_sums = [0]
        for num in nums:
            ps = prefix_sums[-1] + num
            prefix_sums.append(ps)
        
        sorted_prefix_sums = sorted(list(set(prefix_sums + [x - lower for x in prefix_sums] + [x - upper for x in prefix_sums])))
        compress = {v: i for i, v in enumerate(sorted_prefix_sums)}
        
        segment_tree = SegmentTree(len(sorted_prefix_sums))
        count = 0
        
        for ps in prefix_sums:
            left = compress[ps - upper]
            right = compress[ps - lower] + 1
            count += segment_tree.query(left, right) # number of subarray sum <= left, right
            segment_tree.update(compress[ps], 1)
        
        return count
