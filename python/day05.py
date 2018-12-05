#!/bin/env python3

import fileinput
from collections import deque


polymer = fileinput.input()[0].strip()

def react(polymer):
    out = deque()
    for p in polymer:
        if not out:
            out.append(p)
        elif p != out[-1] and p.lower() == out[-1].lower():
            out.pop()
        else:
            out.append(p)
    return out

print("part 1:", len(react(polymer)))
print("part 2:", min(len(react(polymer.replace(p, "").replace(p.upper(), ""))) for p in set(polymer.lower())))