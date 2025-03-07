lines = open("input.txt", "r").readlines()

resultPoints = {
    "B X": 0,
    "A X": 3,
    "C X": 6,
    "C Y": 0,
    "B Y": 3,
    "A Y": 6,
    "A Z": 0,
    "C Z": 3,
    "B Z": 6
}

choicePoints = {
    "X": 1,
    "Y": 2,
    "Z": 3
}

totalScore = 0
for line in lines:
    line = line.replace("\n", "")
    totalScore += resultPoints[line]
    totalScore += choicePoints[line.split(" ")[1]]

print(totalScore)
    