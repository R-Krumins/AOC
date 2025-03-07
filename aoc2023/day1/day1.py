import re
lines = open("input.txt").readlines()
sum = 0

for line in lines:
    line = re.sub("[a-z]", "", line.strip())
    sum += int(line[0] + line[-1])

print(sum)
    

