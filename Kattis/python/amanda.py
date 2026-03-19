# https://open.kattis.com/problems/amanda

import sys
from collections import deque

input = sys.stdin.readline
def main():
    nums = [int(n) for n in input().split(" ")]

    n = nums[0]
    m = nums[1]

    nodes = [-1 for _ in range(n + 1)]
    adj_list = [[] for _ in range(n + 1)]

    # parse nodes
    for _ in range(m):
        nums = [int(n) for n in input().split(" ")]
        e1 = nums[0]
        e2 = nums[1]
        airport = nums[2]
        if airport == 0:
            nodes[e1] = 0
            nodes[e2] = 0
        elif airport == 2:
            nodes[e1] = 1
            nodes[e2] = 1
        else:
            adj_list[e1].append(e2)
            adj_list[e2].append(e1)

    # print(nodes)
    # print(adj_list)

    # Form and check bipartite graph
    for node in range(1, n+1):
        # Visited the node, skip
        if nodes[node] >= 0:
            continue
        q = deque([node])
        while len(q) > 0:
            cn = q.popleft()
            if nodes[cn] >= 0:
                continue
            for neighbour in adj_list[node]:
                if nodes[neighbour] >= 0:
                    # check if this edge joins 2 same color nodes
                    if nodes[neighbour] == nodes[cn]:
                        print("impossible")
                        return
                else:
                    if nodes[cn] >= 0:
                        nodes[neighbour] = 0 if nodes[cn] == 1 else 1
                    q.append(neighbour)
    print(nodes)



if __name__ == "__main__":
    main()
