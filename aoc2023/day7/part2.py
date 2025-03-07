lines = (l.split() for l in open("input.txt").readlines())
cards = []
cardValues= tuple("J23456789TQKA")

for startHand, bid in lines:
    largesType = 0
    for wildCard in cardValues:
        hand = startHand.replace("J", wildCard)
    
        x = "".join(filter(lambda x: hand.count(x) >= 2, hand))
        l = len(x)
        u = len(set(x))        

        if l == 5:
            if u == 1: v = 7
            else: v = 5
        elif l == 4:
            if u == 1: v = 6
            else: v = 3
        elif l == 3: v = 4
        elif l == 2: v = 2
        else: v = 1

        largesType = max(v, largesType)
        
    v = str(largesType)
    for c in startHand:
        v += f"{cardValues.index(c):02}"
    cards.append((startHand, int(bid), int(v)))

cards.sort(key=lambda c: c[2])

sum = 0
for i, card in enumerate(cards):
    sum += card[1] * (i+1)
print(sum)


