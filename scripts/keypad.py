import itertools

NUMPAD_POSITIONS = {
    'A': (0, 2),
    '0': (0, 1),
    '1': (1, 0),
    '2': (1, 1),
    '3': (1, 2),
    '4': (2, 0),
    '5': (2, 1),
    '6': (2, 2),
    '7': (3, 0),
    '8': (3, 1),
    '9': (3, 2),
}

ARROW_POSITIONS = {
    'A': (1, 2),
    '>': (0, 2),
    '^': (1, 1),
    '<': (0, 0),
    'v': (0, 1),
}


def main(positions: dict[str, tuple[int, int]]) -> list[str]:
    combinations = itertools.product(positions.keys(), positions.keys())

    inits = []
    for frm, to in combinations:
        moves = []
        from_positions = positions[frm]
        to_positions = positions[to]

        vertical_diff = to_positions[0] - from_positions[0]
        horizontal_diff = to_positions[1] - from_positions[1]

        if "0" in positions and frm in ("0", "A") and to in ("1", "4", "7"):
            # need to go up first
            if vertical_diff > 0:
                moves.append("^" * vertical_diff)

            if horizontal_diff < 0:
                moves.append("<" * abs(horizontal_diff))

            if vertical_diff < 0:
                moves.append("v" * abs(vertical_diff))

            if horizontal_diff > 0:
                moves.append(">" * horizontal_diff)

        elif "0" in positions and to in ("0", "A") and frm in ("1", "4", "7"):
            # right first
            if horizontal_diff > 0:
                moves.append(">" * horizontal_diff)

            if horizontal_diff < 0:
                moves.append("<" * abs(horizontal_diff))

            if vertical_diff < 0:
                moves.append("v" * abs(vertical_diff))

            if vertical_diff > 0:
                moves.append("^" * vertical_diff)
        elif "^" in positions and frm == "<" and to in ("^", "A"):
            # right first

            if horizontal_diff > 0:
                moves.append(">" * horizontal_diff)

            if horizontal_diff < 0:
                moves.append("<" * abs(horizontal_diff))

            if vertical_diff < 0:
                moves.append("v" * abs(vertical_diff))

            if vertical_diff > 0:
                moves.append("^" * vertical_diff)

        elif "^" in positions and to == "<" and frm in ("^", "A"):
            # down first
            if vertical_diff < 0:
                moves.append("v" * abs(vertical_diff))

            if horizontal_diff < 0:
                moves.append("<" * abs(horizontal_diff))

            if vertical_diff > 0:
                moves.append("^" * vertical_diff)

            if horizontal_diff > 0:
                moves.append(">" * horizontal_diff)
        else:
            if horizontal_diff < 0:
                moves.append("<" * abs(horizontal_diff))

            if vertical_diff < 0:
                moves.append("v" * abs(vertical_diff))

            if vertical_diff > 0:
                moves.append("^" * vertical_diff)

            if horizontal_diff > 0:
                moves.append(">" * horizontal_diff)

        moves.append("A")
        moves = "".join(moves)

        init = f"        ('{frm}', '{to}') => vec![{', '.join(f'\'{mv}\'' for mv in moves)}],"

        inits.append(init)

    return inits
        
        

if __name__ == "__main__":
    print("\n".join(main(NUMPAD_POSITIONS)))
    print("\n\n")
    print("\n".join(main(ARROW_POSITIONS)))
