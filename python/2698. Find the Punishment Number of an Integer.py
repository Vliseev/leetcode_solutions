class Solution:
    def punishmentNumber(self, n: int) -> int:
        def can_partition(square_str: str, target: int, current_sum: int = 0, index: int = 0) -> bool:
            if index == len(square_str):
                return current_sum == target
            num = 0
            for i in range(index, len(square_str)):
                num = num * 10 + int(square_str[i])
                if current_sum + num > target:
                    break
                if can_partition(square_str, target, current_sum + num, i + 1):
                    return True
            return False

        result = 0
        for i in range(1, n + 1):
            square = i * i
            if can_partition(str(square), i):
                result += square
        return result
