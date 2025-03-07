races = zip(*(map(int, l.split(":")[1].split()) for l in open("input.txt")))

totalMultible = 1
for time, distance in races:
    victories = 0
    for t in range(time):
        if (time-t) * t > distance:
            victories += 1
    totalMultible *= victories

print(totalMultible)
           
