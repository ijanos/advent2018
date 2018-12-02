#!/bin/env python3


import fileinput
from collections import Counter

data = [line.strip() for line in fileinput.input()]

twos = 0
threes = 0

for line in data:
    count = Counter(line).values()
    twos += 2 in count
    threes += 3 in count

print(twos * threes)

for i, line1 in enumerate(data):
    for line2 in data[i+1:]:
        out = ""
        diff = 0
        for x, y in zip(line1, line2):
            if x != y:
                diff += 1
            else:
                out += x
            if diff > 1:
                break
        if diff == 1:
            print(out)
