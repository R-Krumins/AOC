import re
import itertools
#parsing
first, second = open("input.txt").read().split("\n\n")
instr = [0 if x == "L" else 1 for x in first]
desertMap = {x[0]: (x[1], x[2]) for x in [re.findall("[A-Z]{3}", y) for y in second.split("\n")]}

#main
node = desertMap["AAA"]
steps = 0

for i in itertools.cycle(instr):
    steps += 1
    if node[i] == "ZZZ": break
    node = desertMap[node[i]]


print(steps)



