import heapq
from collections import defaultdict
from typing import List


class Solution:
    def medianSlidingWindow(self, nums: List[int], k: int) -> List[float]:
        medians = []
        max_heap, min_heap = [], []
        max_heap_size, min_heap_size = 0, 0
        remove_cache = defaultdict(int)

        def find_median():
            """Return the median of the current window"""
            if k % 2 == 1:
                return float(-max_heap[0])
            return (min_heap[0] - max_heap[0]) / 2.0

        def balance_heaps():
            """Balance the heaps to maintain the max-heap size"""
            nonlocal max_heap_size, min_heap_size
            if min_heap_size > max_heap_size:
                heapq.heappush(max_heap, -heapq.heappop(min_heap))
                min_heap_size -= 1
                max_heap_size += 1

        def add_num(num: int):
            """Add a number to the heaps"""
            nonlocal max_heap_size, min_heap_size
            heapq.heappush(max_heap, -num)
            heapq.heappush(min_heap, -heapq.heappop(max_heap))
            min_heap_size += 1
            balance_heaps()

        def remove_num(num: int):
            """Remove a number from the heaps"""
            nonlocal max_heap_size, min_heap_size
            remove_cache[num] += 1
            # num in max heap
            if num <= -max_heap[0]:
                max_heap_size -= 1
            else:
                # num in mean heap
                min_heap_size -= 1

        def clean_heaps():
            """Remove numbers from the heaps that are no longer in the window"""
            nonlocal max_heap_size, min_heap_size
            while max_heap and remove_cache[-max_heap[0]] > 0:
                remove_cache[-max_heap[0]] -= 1
                heapq.heappop(max_heap)
            while min_heap and remove_cache[min_heap[0]] > 0:
                remove_cache[min_heap[0]] -= 1
                heapq.heappop(min_heap)

        # Initialize the heaps with the first k numbers
        for num in nums[:k]:
            add_num(num)

        medians.append(find_median())

        # Slide the window and calculate the median
        for i in range(k, len(nums)):
            remove_num(nums[i - k])
            add_num(nums[i])
            clean_heaps()
            medians.append(find_median())

        return medians


# Test cases
def run_tests():
    solution = Solution()

    # Test case 1: Odd window size
    nums1 = [1, 3, 2, 5, 4]
    k1 = 3
    expected1 = [2.0, 3.0, 4.0]
    result1 = solution.medianSlidingWindow(nums1, k1)
    assert result1 == expected1, f"Test 1 Failed: Expected {expected1}, Got {result1}"

    # Test case 2: Even window size
    nums2 = [1, 2, 3, 4, 5]
    k2 = 2
    expected2 = [1.5, 2.5, 3.5, 4.5]
    result2 = solution.medianSlidingWindow(nums2, k2)
    assert result2 == expected2, f"Test 2 Failed: Expected {expected2}, Got {result2}"

    # Test case 3: Negative numbers
    nums3 = [-1, -2, -3, -4, -5]
    k3 = 3
    expected3 = [-2.0, -3.0, -4.0]
    result3 = solution.medianSlidingWindow(nums3, k3)
    assert result3 == expected3, f"Test 3 Failed: Expected {expected3}, Got {result3}"

    # Test case 4: Single element window
    nums4 = [1, 3, 2, 5, 4]
    k4 = 1
    expected4 = [1.0, 3.0, 2.0, 5.0, 4.0]
    result4 = solution.medianSlidingWindow(nums4, k4)
    assert result4 == expected4, f"Test 4 Failed: Expected {expected4}, Got {result4}"

    # Test case 5: Window size equals array length
    nums5 = [1, 2, 3, 4]
    k5 = 4
    expected5 = [2.5]
    result5 = solution.medianSlidingWindow(nums5, k5)
    assert result5 == expected5, f"Test 5 Failed: Expected {expected5}, Got {result5}"

    # Test case 6: Duplicate numbers
    nums6 = [1, 2, 2, 3, 4]
    k6 = 3
    expected6 = [2.0, 2.0, 3.0]
    result6 = solution.medianSlidingWindow(nums6, k6)
    assert result6 == expected6, f"Test 6 Failed: Expected {expected6}, Got {result6}"

    # Test case 7
    nums7 = [1, 3, -1, -3, 5, 3, 6, 7]
    k7 = 3
    expected7 = [1.00000, -1.00000, -1.00000, 3.00000, 5.00000, 6.00000]
    result7 = solution.medianSlidingWindow(nums7, k7)
    assert result7 == expected7, f"Test 1 Failed: Expected {expected7}, Got {result7}"

    print("All tests passed!")


# Run the tests
run_tests()
