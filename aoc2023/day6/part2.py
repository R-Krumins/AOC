time, distance = map(int, [l.split(":")[1].replace(" ", "") for l in open("input.txt")])

victories = 0
for t in range(time):
    if (time-t) * t > distance:
        victories += 1

print(victories)
           
