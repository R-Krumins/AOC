#parsing
parts = open("input.txt").read().split("\n\n")
seeds = [int(i) for i in parts.pop(0)[7:].split()]

maps = []
for p in parts:
    l = [x.split() for x in p.split("\n")]
    l.pop(0)
    maps.append(l)

#main shit
destSList = []
for s in seeds:
    destS = s
    for m in maps:
        for l in m:
            dest = int(l[0])
            src = int(l[1])
            range = int(l[2])

            if src <= destS < src + range:
                destS = destS + (dest - src)
                break

    destSList.append(destS)

print(min(destSList))