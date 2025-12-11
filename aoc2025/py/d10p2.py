import numpy as np
# with open("../inputs/d10.txt", "r") as f:
with open("../inputs/input", "r") as f:
    _input = f.readlines()
    for line in _input:
        line = line[:-1]
        end_state = [int(n) for n in line.split(" {")[1][:-1].split(",")]
        toggles = line.split("] ")[1].split(" {")[0].split(" ")
        toggles = [set([int(t) for t in toggle[1:-1].split(",")]) for toggle in toggles]
        # print(toggles)
        for _ in range(len(toggles) - len(end_state)):
            end_state.append(0)
        new_toggles = []
        for toggle in toggles:
            t = []
            for i in range(len(end_state)):
                if i in toggle:
                    t.append(1)
                else:
                    t.append(0)
            for _ in range(len(toggles) - len(end_state)):
                t.append(0)
            new_toggles.append(t)
        toggles = np.array(new_toggles).transpose()
        print(toggles)
        print(toggles.shape)
        end_state = np.array([end_state])
        print(end_state.shape)
        print(end_state)
        sol = np.linalg.solve(toggles, end_state)
        print(sol)
