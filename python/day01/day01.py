#!/bin/env python3

import fileinput
from itertools import cycle

data = [int(line.strip()) for line in fileinput.input()]

print("part 1:", sum(data))

freq = 0
seen = set()

for dirft in  cycle(data):
    freq += dirft
    if freq in seen:
        break
    seen.add(freq)

print("part 2:", freq)