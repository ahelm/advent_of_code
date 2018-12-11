from pathlib import Path

CWD = Path(__file__).parent

with open(CWD / "puzzle_input.txt") as f:
    lines = f.readlines()

freq = 0
all_freq = set((0,))
for i, line in enumerate(lines * 1000, start=2):
    freq += int(line)
    all_freq.add(freq)
    if len(all_freq) != i:
        print(freq)
        break
