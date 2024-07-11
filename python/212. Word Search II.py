from typing import List


class TrieNode:
    def __init__(self):
        self.children = {}
        self.is_word = False


class Solution:
    def findWords(self, board: List[List[str]], words: List[str]) -> List[str]:
        # Build the trie
        root = TrieNode()
        for word in words:
            node = root
            for char in word:
                if char not in node.children:
                    node.children[char] = TrieNode()
                node = node.children[char]
            node.is_word = True

        # DFS function
        def dfs(i, j, node: TrieNode, path: str):
            if node.is_word:
                result.add(path)
                node.is_word = False
            if i < 0 or i >= m or j < 0 or j >= n:
                return
            ch = board[i][j]

            if not ch in node.children:
                return
            board[i][j] = "#"
            for di, dj in ((-1, 0), (0, 1), (1, 0), (0, -1)):
                dfs(i + di, j + dj, node.children[ch], path + ch)
            board[i][j] = ch

        # Main logic
        m, n = len(board), len(board[0])
        result = set()

        for i in range(m):
            for j in range(n):
                dfs(i, j, root, "")

        return list(result)


# Test cases
def run_test_case(board, words, expected_output):
    solution = Solution()
    result = solution.findWords(board, words)
    result.sort()  # Sort for consistent comparison
    expected_output.sort()

    if result == expected_output:
        print("Test case passed!")
    else:
        print("Test case failed.")
        print(f"Expected: {expected_output}")
        print(f"Got: {result}")
    print()


# Test case 1
board1 = [
    ["o", "a", "a", "n"],
    ["e", "t", "a", "e"],
    ["i", "h", "k", "r"],
    ["i", "f", "l", "v"],
]
words1 = ["oath", "pea", "eat", "rain"]
expected1 = ["eat", "oath"]
run_test_case(board1, words1, expected1)

# Test case 2
board2 = [["a", "b"], ["c", "d"]]
words2 = ["abcb"]
expected2 = []
run_test_case(board2, words2, expected2)

# Test case 3
board3 = [
    ["o", "a", "b", "n"],
    ["o", "t", "a", "e"],
    ["a", "h", "k", "r"],
    ["a", "f", "l", "v"],
]
words3 = ["oa", "oaa"]
expected3 = ["oa", "oaa"]
run_test_case(board3, words3, expected3)

# Test case 4
board4 = [["a", "b", "c"], ["a", "e", "d"], ["a", "f", "g"]]
words4 = ["abcdefg", "gfedcbaaa", "eaabcdgfa", "befa", "dgc", "ade"]
expected4 = ["abcdefg", "befa", "eaabcdgfa","gfedcbaaa"]
run_test_case(board4, words4, expected4)
