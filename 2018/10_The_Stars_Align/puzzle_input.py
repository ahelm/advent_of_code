from test_input import parse_input, visualize, CWD, get_volume, MAX_INT


with open(CWD / "puzzle_input.txt", mode="r") as fp:
    pos, vel = parse_input(fp.readlines())
    vol_max = MAX_INT
    for i in range(20000):
        current_vol = get_volume(pos)
        print(f">> Iteration: {i}: {current_vol}")
        if current_vol >= vol_max:
            # unset previous step
            pos -= vel
            visualize(pos, i - 1, "puzzle_solution_")
            break
        vol_max = current_vol
        pos += vel

