class Solution:
    def minGroups(self, intervals: List[List[int]]) -> int:
        intervals.sort()

        max_groups = 0
        heap = []

        for (beg, end) in intervals:
            while heap and heap[0] < beg:
                heapq.heappop(heap)
            heapq.heappush(heap, end)
            max_groups = max(max_groups, len(heap))
        return max_groups
