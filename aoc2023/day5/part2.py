#parsing
parts = open("input.txt").read().split("\n\n")
seeds = [int(i) for i in parts.pop(0)[7:].split()]

mapsRaw = [p.split("\n")[1:] for p in parts]
maps = []
for m in mapsRaw:
    maps.append([tuple(map(int, s.split())) for s in m])



#find the correct seed range
newSeeds = []
for i in range(0, len(seeds), 2):
    newSeeds.append(seeds[i])
    newSeeds.append(seeds[i] + seeds[i+1] - 1)

results = []
for i, s in enumerate(newSeeds):
    destS = s
    for map in maps:
        for dest, src, rng in map:
            if src <= destS < src + rng:
                destS = destS + (dest - src)
                break
        
    results.append((i, destS))


seedIndex, resultSeed = min(results, key=lambda i: i[1])

#find aproximate seed
minS = newSeeds[seedIndex-1]
maxS = newSeeds[seedIndex]

results = []
accuracy = 10000
for s in range(minS, maxS, accuracy):
    destS = s
    for map in maps:
        for dest, src, rng in map:
            if src <= destS < src + rng:
                destS = destS + (dest - src)
                break

    results.append((s, destS))
    print(s, "/", maxS)



# find real seed
seed, resultSeed = min(results, key=lambda i: i[1])
minS = seed - accuracy
maxS = seed + accuracy

results = []
for s in range(minS, maxS):
    destS = s
    for map in maps:
        for dest, src, rng in map:
            if src <= destS < src + rng:
                destS = destS + (dest - src)
                break

    results.append(destS)
    print(s, "/", maxS)

print("true result:", min(results))








