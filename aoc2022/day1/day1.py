file = open("calories.txt", "r")
lines = file.readlines()

biggestSum = 0
currentSum = 0

for line in lines:
    if line == "\n":
        if currentSum > biggestSum: biggestSum = currentSum
        currentSum = 0
        continue
    
    currentSum += int(line)

print(biggestSum)
