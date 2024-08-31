import heapq
from typing import List

class Solution:
    def maxProbability(self, n: int, edges: List[List[int]], succProb: List[float], start_node: int, end_node: int) -> float:
        graph = [[] for _ in range(n)]
        for prob, (a, b) in zip(succProb, edges):
            graph[a].append((b, prob))
            graph[b].append((a, prob))

        probabilities = [0] * n
        probabilities[start_node] = 1

        # Priority queue to store (-probability, node)
        pq = [(-1, start_node)]

        while pq:
            prob, node = heapq.heappop(pq)
            prob = - prob
            if node == end_node:
                return prob

            if prob < probabilities[node]:
                continue

            for neigh, neigh_prob in graph[node]:
                new_prob = prob * neigh_prob
                if new_prob > probabilities[neigh]:
                    probabilities[neigh] = new_prob
                    heapq.heappush(pq, (-new_prob, neigh))
        return 0
