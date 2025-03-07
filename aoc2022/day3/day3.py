lines = open("input.txt", "r").readlines()

def findCommonItem(firstpart, secondpart):
    for x in firstpart:
        for y in secondpart:
            if x == y:
                return x

commonItemList = []
for line in lines:
    line = line.strip()
    commonItemList.append(findCommonItem(line[:len(line)//2], line[len(line)//2:]))

prioritySum = 0
for item in commonItemList:
    prioritySum += ord(item)-38 if item.isupper() else ord(item)-96

print(prioritySum)





    


