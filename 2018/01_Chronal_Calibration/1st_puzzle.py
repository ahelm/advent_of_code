from pathlib import Path

CWD = Path(__file__).parent

with open(CWD / "puzzle_input.txt") as f:
    freq = 0
    for l in f.readlines():
        freq += int(l)

print(freq)

