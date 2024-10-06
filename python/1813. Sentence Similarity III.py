class Solution:
    def areSentencesSimilar(self, sentence1: str, sentence2: str) -> bool:
        words1 = sentence1.split(" ")
        words2 = sentence2.split(" ")

        n, m = len(words1), len(words2)
        i = j = 0

        while i < len(words1) and i < len(words2) and words1[i] == words2[i]:
            i += 1
        while j < len(words1) - i and j < len(words2) - i and words1[n - 1 - j] == words2[m - 1 - j]:
            j += 1

        return i + j >= n or i + j >= m
