lines = open("input.txt", "r").readlines()

def findCommonInThree(first, second, third):
    for c1 in first:
        for c2 in second:
            for c3 in third:
                if c1 == c2 and c2 == c3:
                    return ord(c3)-38 if c3.isupper() else ord(c3)-96

prioritySum = 0
for i in range(0, len(lines), 3):
    prioritySum += findCommonInThree(lines[i].strip(), lines[i+1].strip(), lines[i+2].strip())

print(prioritySum)






    


