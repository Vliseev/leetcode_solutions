class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        if len(nums1) > len(nums2):
            nums1, nums2 = nums2, nums1

        m, n = len(nums1), len(nums2)
        low, high = 0, m

        while low <= high:
            part1 = (low + high) // 2
            part2 = (m + n + 1) // 2 - part1

            max_left1 = float('-inf') if part1 == 0 else nums1[part1 - 1]
            min_right1 = float('inf') if part1 == m else nums1[part1]

            max_left2 = float('-inf') if part2 == 0 else nums2[part2 - 1]
            min_right2 = float('inf') if part2 == n else nums2[part2]
            if max_left1 <= min_right2 and max_left2 <= min_right1:
                if (m + n) % 2 == 0:
                    return (max(max_left1, max_left2) + min(min_right1, min_right2)) / 2
                else:
                    return max(max_left1, max_left2)
            elif max_left1 > min_right2:
                high = part1 - 1
            else:
                low = part1 + 1


        raise ValueError("Input arrays are not sorted")
