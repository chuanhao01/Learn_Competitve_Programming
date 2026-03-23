# https://open.kattis.com/problems/

import sys, math
from collections import deque

input = sys.stdin.readline
def main():
    while True:
        C = int(input())
        if C == 0:
            break
        code_to_node = {}
        codes = input().strip().split(" ")
        adj_list = [[] for _ in range(C)]
        for i in range(C):
            code_to_node[codes[i]] = i
        R = int(input())
        for _ in range(R):
            l = input().strip().split(" ")
            e1 = code_to_node[l[0]]
            e2 = code_to_node[l[1]]
            nums = [int(num) for num in l[2].split(":")]
            weight = -math.log(nums[1]/nums[0])
            adj_list[e1].append((e2, weight))
        if bellman_ford(C, adj_list):
            print("Arbitrage")
        else:
            print("Ok")



def bellman_ford(n: int, adj_list: list[tuple[int, float]]) -> bool:
    """
    Checks if there is a negative cycle in the graph
    """
    for cn in range(n):
        weights = [None for _ in range(n)]
        weights[cn] = 0
        # relax all edges n-1 times
        for _ in range(n-1):
            for node, node_neighbours in enumerate(adj_list):
                for edge in node_neighbours:
                    # Skip if the weight is INF
                    if weights[node] is None:
                        continue
                    # Else try and make it shorter
                    if weights[edge[0]] is None:
                        weights[edge[0]] = weights[node] + edge[1]
                    else:
                        weights[edge[0]] = min(weights[edge[0]], weights[node] + edge[1])
        # print(weights)
        # If we try and relax it one more time check if we can
        for node, node_neighbours in enumerate(adj_list):
            for edge in node_neighbours:
                # Skip if the weight is INF
                if weights[node] is None:
                    continue
                # Else try and make it shorter
                if weights[edge[0]] is None:
                    # I think there is an error? This should not happen i think
                    return True
                elif weights[node] + edge[1] < weights[edge[0]]:
                    return True

    return False


if __name__ == "__main__":
    main()
