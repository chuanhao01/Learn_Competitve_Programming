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
    nodes_to_check = [0 for _ in range(n + 1)]

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
            nodes_to_check[e1] = 1
            nodes_to_check[e2] = 1

    # print(nodes)
    # print(adj_list)

    # print(nodes_to_check)
    # Form and check bipartite graph
    visited = [False for _ in range(n + 1)]
    for i in range(n+1):
        if nodes_to_check[i] == 0:
            continue
        if visited[i]:
            continue

        # BFS to find the first node with a known value
        q = deque([i])
        start_node = None
        while len(q) > 0:
            node = q.popleft()
            if visited[i]:
                continue
            visited[i] = True
            if nodes[node] != -1:
                start_node = node
                break
            for neighbour in adj_list[node]:
                q.append(neighbour)

        # Bipartite and flip all
        if start_node is None:
            continue
        new_visited = [False for _ in range(n + 1)]
        q = deque([start_node])
        while len(q) > 0:
            node = q.popleft()
            if new_visited[node]:
                continue
            new_visited[node] = True
            for neighbour in adj_list[node]:
                if nodes[neighbour] == nodes[node]:
                    print("impossible")
                    return
                if new_visited[neighbour]:
                    continue
                else:
                    nodes[neighbour] = 0 if nodes[node] == 1 else 1
                    q.append(neighbour)

    all_ones = 0
    for c in nodes:
        if c == 1:
            all_ones += 1

    # print(nodes)
    final_nodes = []
    # Final bipartite
    for i in range(1, n+1):
        if nodes[i] == -1:
            final_nodes.append(i)
    # print(final_nodes)
    min_counts = 0
    new_visited = [False for _ in range(n + 1)]
    for i in final_nodes:
        if new_visited[i]:
            continue
        counts = [-1 for _ in range(n+1)]
        counts[i] = 0
        q = deque([i])
        while len(q) > 0:
            node = q.popleft()
            if new_visited[node]:
                continue
            new_visited[node] = True
            for neighbour in adj_list[node]:
                if counts[neighbour] == counts[node]:
                    print("impossible")
                    return
                if new_visited[neighbour]:
                    continue
                else:
                    counts[neighbour] = 0 if counts[node] == 1 else 1
                    q.append(neighbour)
        zeros = 0
        ones = 0
        for c in counts:
            if c == 0:
                zeros += 1
            elif c == 1:
                ones += 1
        min_counts += min(zeros, ones)
    # print(min_counts + all_ones)
    print(all_ones)




if __name__ == "__main__":
    main()
