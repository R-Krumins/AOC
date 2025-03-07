lines = open("input.txt").readlines()


overlapCount = 0
for line in lines:
    line = line.strip()
    pair = line.split(",")
    elf1, elf2 = pair[0].split("-"), pair[1].split("-")

    if elf1[0]<elf2[0] and elf1[1]>elf2[1]: overlapCount += 1
    elif elf1[0]>elf2[0] and elf1[1]<elf2[1]: overlapCount += 1

print(overlapCount)