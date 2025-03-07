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

remap = {
    "A X": "A Z",
    "A Y": "A X",
    "A Z": "A Y",
    "B X": "B X",
    "B Y": "B Y",
    "B Z": "B Z",
    "C X": "C Y",                             
    "C Y": "C Z",
    "C Z": "C X"
}

totalScore = 0
for line in lines:
    line = line.replace("\n", "")
    line = remap[line]
    totalScore += resultPoints[line]
    totalScore += choicePoints[line.split(" ")[1]]

print(totalScore)
    