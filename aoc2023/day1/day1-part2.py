import re
lines = open("input.txt").readlines()
sum = 0
numbMap = {"one": "1", "two": "2", "three": "3", "four": "4", "five": "5", "six": "6", "seven": "7", "eight": "8", "nine": "9"}

for line in lines:
    numbers = re.findall("(?=([1-9]|one|two|three|four|five|six|seven|eight|nine))", line)
    numb1 = numbers[0] if numbers[0].isnumeric() else numbMap[numbers[0]]
    numb2 = numbers[-1] if numbers[-1].isnumeric() else numbMap[numbers[-1]]
    sum += int(numb1 + numb2)

print(sum)