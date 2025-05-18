BOARD_SIZE = 8

# Checks if the current tile (row, col) is not attacked by any other queen inside 'satate'.
def is_safe(state, row, col):
    for r in range(row):
        c = state[r]
        # >> if the cols are the same
        # >> if the diagonals are the same
        # NOTE: The rows will NEVER be the same since they're represented as each element in the array.
        if c == col or abs(c - col) == abs(r - row):
            return False
    return True

def validate_initial_state(initial_state):
    for i in range(len(initial_state)):
        if not is_safe(initial_state, i, initial_state[i]):
            raise Exception("Invalid initial state: Queens are attacking each other.")

def eight_queens(initial_state):
    validate_initial_state(initial_state)

    found = False
    # (queens), (how many queens are there) 
    stack = [(initial_state, len(initial_state))]
    nodes_created = 0

    while stack and not found:
        state, row = stack.pop()
        nodes_created += 1

        if row == BOARD_SIZE:
            found = True
            continue # found one solution

        # NOTE: reverse for "left-to-right"
        for col in range(BOARD_SIZE-1, -1, -1):
            if is_safe(state, row, col):
                stack.append((state + [col], row + 1))

    print("-" * 30)
    print("initial_state:")
    print(initial_state)
    print("# VEREDICT #")
    if found:
        print(f"Total nodes created: {nodes_created}")
        print("Valid 8-Queens solution found:")
        print_board(state)
    else:
        print("No solution found.")
        print(f"Total nodes created: {nodes_created}")

def print_board(state):
    print(end='  ')
    print(' '.join([str(n) for n in range(BOARD_SIZE)]))
    for row in range(BOARD_SIZE):
        line = ['.'] * BOARD_SIZE
        line[state[row]] = 'Q'
        print(str(row), end=' ')
        print(' '.join(line))

if __name__ == "__main__":
    # NOTE: Each element in the array represents a ROW.
    eight_queens([]) # no queens
    eight_queens([4]) # one queen (0x4)
    eight_queens([0, 5]) # two queens (0x0, 1x5)
    eight_queens([0, 4, 7]) # three queens (0x0, 1x4, 2x7)
