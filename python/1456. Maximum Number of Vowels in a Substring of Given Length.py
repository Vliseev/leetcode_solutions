class Solution:
    def maxVowels(self, s: str, k: int) -> int:
        vowels = set('aeiou')
        max_count = cur_count = sum(1 for c in s[:k] if c in vowels)
        
        for i in range(k, len(s)):
            cur_count = cur_count - (s[i - k] in vowels) + (s[i] in vowels)
            max_count = max(max_count, cur_count)
        
        return max_count
