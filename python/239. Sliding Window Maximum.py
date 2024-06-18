class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        deq = deque()
        result = []
        for i in range(len(nums)):
            while deq and deq[0] < i - k + 1:
                deq.popleft()
            # We then remove elements from the back of the deque while the current element is larger than those in the deque.
            # This ensures that the elements in the deque are always in decreasing order.
            while deq and nums[deq[-1]] < nums[i]:
                deq.pop()
            deq.append(i)
            if i >= k - 1:
                result.append(nums[deq[0]])
        return result
