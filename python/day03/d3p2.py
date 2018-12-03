#!/bin/env python3

import fileinput
import re
from collections import defaultdict


textil = defaultdict(list)
part2 = set()

for line in fileinput.input():
    id, _, x, y, _, w, h = re.split(",|x|:| ", line.strip())
    x, y, w, h = (int(n) for n in (x, y, w, h))
    potential = True
    for cx in range(x, x + w):
        for cy in range(y, y + h):
            claimed = textil[(cx, cy)]
            for claim in claimed:
                if claim in part2:
                    part2.remove(claim)
                potential = False
            textil[(cx,cy)].append(id)
    if potential:
        part2.add(id)

print("part 1:", len(list(filter(lambda s: len(s) > 1, textil.values()))))
print("part 2:", part2.pop())
