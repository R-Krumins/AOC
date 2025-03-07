import re, itertools, math
#parsing
first, second = open("input.txt").read().split("\n\n")
instr = [0 if x == "L" else 1 for x in first]
desertMap = {x[0]: (x[1], x[2]) for x in [re.findall("\w{3}", y) for y in second.split("\n")]}

#main
nodes = [desertMap[k] for k in desertMap if re.match("\w\wA", k)]
leastSteps = []

for n in nodes:
    steps = 0
    for i in itertools.cycle(instr):
        steps += 1
        if re.match("\w\wZ", n[i]):
            leastSteps.append(steps)
            break
        n = desertMap[n[i]]

print(math.lcm(*leastSteps))



