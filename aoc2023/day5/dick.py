#parsing
parts = open("inputTest.txt").read().split("\n\n")
seeds = [int(i) for i in parts.pop(0)[7:].split()]

mapsRaw = [p.split("\n")[1:] for p in parts]
maps = []
for m in mapsRaw:
    maps.append([tuple(map(int, s.split())) for s in m])

destSList = []
for i in range(0, len(seeds), 2):
    skip = 1
    for seed in range(seeds[i], seeds[i] + seeds[i+1], skip):
        destS = seed
        for map in maps:
            for dest, src, rng in map:
                if src <= destS < src + rng:
                    destS = destS + (dest - src)
                    break
        destSList.append(destS)
        print(f"Seed: {seed} Output: {destS}")
    print("\n")
print("Result:", min(destSList))