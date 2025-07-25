from collections import deque

MATRIX_DIMS = 3 # 3x3

def is_objective(state, goal_state):
    return state == goal_state

def create_children_and_add_to_list(state, move_count, queue):
    index = state.index(0) 
    # divmod returns a tuple containing the quotient and remainder when dividing a by b.
    row, col = divmod(index, MATRIX_DIMS)
    
    # NOTE: 
    # >> When the empty tile is at the center, it will have 4 children (nodes).
    # >> When it's a the center borders, it will have 3 children.
    # >> When it's at the corners, it will have 2 children.
    directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    
    for dr, dc in directions:
        new_row, new_col = row + dr, col + dc
        if 0 <= new_row < MATRIX_DIMS and 0 <= new_col < MATRIX_DIMS:
            # NOTE: new_row * MATRIX_DIMS because it's a FLAT tuple, not a matrix.
            new_index = new_row * MATRIX_DIMS + new_col
            # NOTE: At this point, new_state is still just a copy of the old state.
            new_state = list(state)
            new_state[index], new_state[new_index] = new_state[new_index], new_state[index]
            queue.append((tuple(new_state), move_count + 1))

def puzzle(initial_state, goal_state):
    found = False
    # It's BFS because it's a queue, not a stack (DFS).
    # (state, move_count)
    nodeQueue = deque([(initial_state, 0)])
    dictVisited = dict()

    while not found and nodeQueue:
        node, moves = nodeQueue.popleft()
        if node not in dictVisited:
            dictVisited[node] = True
            if is_objective(node, goal_state):
                found = True
                # continue
            else:
                create_children_and_add_to_list(node, moves, nodeQueue)
    
    print("-"*42)
    print("initial_state: ", end="")
    print(initial_state)
    print("# VEREDICT #")
    if found:
        print("Goal state found:")
        print(node)
        print(f"Number of moves: {moves}")
        # for k in dictVisited.keys(): print(k)
    else:
        print("No solutions")

if __name__ == "__main__":
    initial_states = [
        (4, 6, 2, 8, 1, 3, 7, 5, 0),
        (6, 4, 2, 8, 1, 3, 7, 5, 0),
        (1, 2, 3, 4, 5, 6, 7, 8, 0),
        (7, 5, 4, 1, 0, 3, 2, 6, 8),
    ]
    goal_state = (0, 1, 2, 
                  3, 4, 5, 
                  6, 7, 8)
    for state in initial_states:
        puzzle(state, goal_state)
    
