from collections import Counter


class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        n = len(nums)
        cntr = Counter(nums)
        cntr_list = [[] for _ in range(n + 1)]
        for el in cntr:
            num_el = cntr[el]
            cntr_list[num_el].append(el)
        result = []

        for glob_id in range(n, -1, -1):
            for loc_id in range(len(cntr_list[glob_id]) - 1, -1, -1):
                result.append(cntr_list[glob_id][loc_id])
                if len(result) == k:
                    return result
        return result
