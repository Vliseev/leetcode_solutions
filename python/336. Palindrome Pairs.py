class Solution:
    def palindromePairs(self, words: List[str]) -> List[List[int]]:
        word_dict = {word: i for i, word in enumerate(words)}
        result = []

        def is_palindrome(word: str) -> bool:
            return word == word[::-1]

        for i, word in enumerate(words):
            for j in range(len(word)+1): # empty word
                prefix = word[:j]
                suffix = word[j:]

                rev_prefix = prefix[::-1]
                rev_suffix = suffix[::-1]
                if is_palindrome(suffix):
                    if rev_prefix in word_dict and word_dict[rev_prefix] != i:
                        result.append([i, word_dict[rev_prefix]])
                if j != 0 and is_palindrome(prefix): #skip duplicate j = len(word) + 1
                    if rev_suffix in word_dict and word_dict[rev_suffix] != i:
                        result.append([word_dict[rev_suffix], i])
        return result
