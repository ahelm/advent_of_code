from pathlib import Path
import re

import numpy as np
import matplotlib.pyplot as plt
import imageio
import sys

CWD = Path(__file__).parent
MAX_INT = sys.maxsize


def parse_input(lines):
    pos = []
    vel = []
    for l in lines:
        matches = re.search(
            r"position=<([\s-]*[0-9]+,[\s-]*[0-9]+)> "
            + r"velocity=<([\s-]*[0-9]+,[\s-]*[0-9]+)>",
            l,
        )
        pos.append(matches.group(1).split(","))
        vel.append(matches.group(2).split(","))

    return np.array(pos, dtype=int), np.array(vel, dtype=int)


def get_volume(pos):
    xmin = pos[:, 0].min()
    ymin = pos[:, 1].min()

    xmax = pos[:, 0].max()
    ymax = pos[:, 0].max()

    return np.abs(xmax - xmin) * np.abs(ymax - ymin)


def visualize(pos, iteration, name_prefix=""):
    # normalize to canvas (0, 0)
    pos[:, 0] -= pos[:, 0].min()
    pos[:, 1] -= pos[:, 1].min()
    canvas = np.zeros(pos.max(axis=0) + 1, dtype=np.uint8)

    canvas[pos[:, 0], pos[:, 1]] = 1

    # matplotlib visualization
    plt.imshow(canvas.T, cmap="binary")
    plt.axis("off")
    plt.tight_layout(pad=0.0)
    plt.savefig(CWD / "output" / f"{name_prefix}{iteration:05d}.png", dpi=100)


if __name__ == "__main__":
    with open(CWD / "test_input.txt", mode="r") as fp:
        pos, vel = parse_input(fp.readlines())
        vol_max = MAX_INT
        for i in range(5):
            current_vol = get_volume(pos)
            print(f">> Iteration: {i}: {current_vol}")
            if current_vol >= vol_max:
                # unset previous step
                pos -= vel
                visualize(pos, i - 1, "test_case_")
                break
            vol_max = current_vol
            pos += vel
