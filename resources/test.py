letters = {

}

with open('words_alpha.txt') as f:
    line = f.readline()

    while line != "":

        for i in line:
            if i == "\n" or i == "\r":
                continue
            if i in letters:
                letters[i] += 1
            else:
                letters[i] = 1
        line = f.readline()
    

total = 0
for i in letters:
    total += letters[i]

for i in letters: 
    print(i, (letters[i] / total) * 1000 )