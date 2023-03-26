def translate(x: str):
    return { 'A': 'R', 'B': 'P', 'C': 'S', 'X': 'R', 'Y': 'P', 'Z': 'S', }[x]

def score(line: str):
    theirs, mine = line.split(' ')
    theirs, mine = translate(theirs), translate(mine)
    s1 = { 'R': 1, 'P': 2, 'S': 3 }[mine]
    i_win = (mine, theirs) in [('R', 'S'), ('S', 'P'), ('P', 'R')]
    draw = mine == theirs
    s2 = 6 if i_win else 3 if draw else 0
    return s1 + s2

with open('src/d02/input') as f:
    lines = (line.strip() for line in f.readlines())
    print(sum(score(line) for line in lines))
