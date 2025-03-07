file = open("calories.txt", "r")
lines = file.readlines()

calories = []

sum = 0
for line in lines:
    if line == "\n":
        calories.append(sum)
        sum = 0
        continue
    
    sum += int(line)

calories.sort(reverse=True)

top3Sum = calories[0] + calories[1] + calories[2]

print(top3Sum)
