from typing import List
import heapq


class Solution:
    def smallestRange(self, nums: List[List[int]]) -> List[int]:
        hp = [(el[0], 0, id) for id, el in enumerate(nums)]
        heapq.heapify(hp)
        max_el = max(el[0] for el in nums)

        left_boun, right_bound = float("-inf"), float("+inf")
        while hp:
            min_el, pos, id = heapq.heappop(hp)
            if max_el - min_el < right_bound - left_boun:
                left_boun, right_bound = min_el, max_el
            elif (
                max_el - min_el == right_bound - left_boun
                and max_el < right_bound
                and min_el > left_boun
            ):
                left_boun, right_bound = min_el, max_el
            if pos == len(nums[id]):
                break
            next_el = nums[id][pos]
            max_el = max(next_el, max_el)
            heapq.heappush(hp, (next_el, pos + 1, id))
        return [left_boun, right_bound]


# Example usage
nums1 = [[4, 10, 15, 24, 26], [0, 9, 12, 20], [5, 18, 22, 30]]
nums2 = [[1, 2, 3], [1, 2, 3], [1, 2, 3]]
print(Solution().smallestRange(nums1))  # Output: [20, 24]
print(Solution().smallestRange(nums2))  # Output: [1, 1]
