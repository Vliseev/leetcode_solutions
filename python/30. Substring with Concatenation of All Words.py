from typing import List


class Solution:
    def findSubstring(self, s: str, words: List[str]) -> List[int]:
        if not s or not words:
            return []
        word_len = len(words[0])
        n_words = len(words)
        tot_len = n_words * word_len
        wc = {}
        for w in words:
            wc[w] = wc.get(w, 0) + 1

        result_ind = []

        for offset in range(word_len):
            cntr = {}
            left = offset
            formed = 0
            for right in range(offset, len(s) - word_len + 1, word_len):
                w = s[right : right + word_len]
                if w in wc:
                    cntr[w] = cntr.get(w, 0) + 1
                    formed += 1
                    while cntr[w] > wc[w]:
                        lw = s[left : left + word_len]
                        left += word_len
                        cntr[lw] -= 1
                        formed -= 1
                    if formed == n_words:
                        result_ind.append(left)
                else:
                    cntr.clear()
                    formed = 0
                    left = right + word_len
        return result_ind


def run_test(test_name, s, words, expected):
    solution = Solution()
    result = solution.findSubstring(s, words)
    if set(result) == set(expected):
        print(f"{test_name} - Passed")
    else:
        print(f"{test_name} - Failed")
        print(f"  Expected: {expected}")
        print(f"  Got: {result}")


# Test cases
def run_all_tests():
    # Test 1
    run_test("Example 1", "barfoothefoobarman", ["foo", "bar"], [0, 9])

    # Test 2
    run_test(
        "Example 2", "wordgoodgoodgoodbestword", ["word", "good", "best", "word"], []
    )

    # Test 3
    run_test("Example 3", "barfoofoobarthefoobarman", ["bar", "foo", "the"], [6, 9, 12])

    # Test 4
    run_test("Empty String", "", ["foo", "bar"], [])

    # Test 5
    run_test("Empty Words", "foobarthebarfoo", [], [])

    # Test 6
    run_test("Single Word", "aaa", ["a"], [0, 1, 2])

    # Test 7
    run_test("No Match", "abcdef", ["xyz"], [])

    # Test 8
    run_test("Overlapping Words", "aaaaaa", ["aa", "aa"], [0, 2])

    # Test 9
    run_test("Repeated Words", "foobarfoobar", ["foo", "bar"], [0, 3, 6])


if __name__ == "main":
    run_all_tests()
