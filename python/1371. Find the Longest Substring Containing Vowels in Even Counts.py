from collections import defaultdict

class Solution:
    def findTheLongestSubstring(self, s: str) -> int:
        vowels_dict = {'a': 0, 'e': 1, 'i': 2, 'o': 3, 'u': 4}
        mask_to_index = {0: -1}
        mask = 0
        max_length = 0

        for i, ch in enumerate(s):
            if ch in vowels_dict:
                mask ^= (1 << vowels_dict[ch])
            if mask in mask_to_index:
                max_length = max(max_length, i - mask_to_index[mask])
            else:
                mask_to_index[mask] = i

        return max_length
