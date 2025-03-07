#parsing
parts = open("inputTest.txt").read().split("\n\n")
seeds = [int(i) for i in parts.pop(0)[7:].split()]

maps = []
for p in parts:
    l = [x.split() for x in p.split("\n")]
    l.pop(0)
    maps.append(l)

newSeeds = []
for i in range(0, len(seeds), 2):
    newSeeds.append(seeds[i])
    newSeeds.append(seeds[i] + seeds[i+1] - 1)

#main shit
destSList = []
for s in newSeeds:
    print("\nseed:", s)
    destS = s
    for m in maps:
        for l in m:
            dest = int(l[0])
            src = int(l[1])
            range = int(l[2])

            if src <= destS < src + range:
                print("src:",destS, end=" ")
                destS = destS + (dest - src)
                print("dest:", destS, f"({dest - src})")
                
                break

    destSList.append(destS)

print(min(destSList))