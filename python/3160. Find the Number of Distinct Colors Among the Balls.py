class Solution:
    def queryResults(self, limit: int, queries: List[List[int]]) -> List[int]:
        ball_dict = {}
        color_dict = {}

        result = []

        for (ball, colour) in queries:
            if ball in ball_dict:
                color_ball = ball_dict[ball]
                color_dict[color_ball] -= 1
                if color_dict[color_ball] == 0:
                    del color_dict[color_ball]
                color_dict[colour] = color_dict.get(colour, 0) + 1
                ball_dict[ball] = colour
            else:
                ball_dict[ball] = colour
                color_dict[colour] = color_dict.get(colour, 0) + 1
            result.append(len(color_dict))
        return result
