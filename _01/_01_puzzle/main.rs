use std::collections::{HashSet, VecDeque};

const MATRIX_DIMS: usize = 3; // 3x3

// YES, element wise comparison thanks to PartialEq trait
fn is_objective(state: &[u8], goal_state: &[u8]) -> bool {
    state == goal_state
}

fn create_children_and_add_to_queue(
    state: &[u8],
    move_count: usize,
    queue: &mut VecDeque<(Vec<u8>, usize)>,
) {
    let index: usize = state.iter().position(|&x| x == 0).unwrap();
    let row: usize = index / MATRIX_DIMS; // quocient
    let col: usize = index % MATRIX_DIMS; // rest

    // NOTE:
    // >> When the empty tile is at the center, it will have 4 children (nodes).
    // >> When it's at the center borders, it will have 3 children.
    // >> When it's at the corners, it will have 2 children.
    let directions: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (dr, dc) in directions.iter() {
        let new_row = row as i8 + dr;
        let new_col = col as i8 + dc;

        if new_row >= 0
            && new_row < MATRIX_DIMS as i8
            && new_col >= 0
            && new_col < MATRIX_DIMS as i8
        {
            // NOTE: new_row * MATRIX_DIMS because it's a FLAT vector, not a matrix.
            let new_index: usize = (new_row as usize) * MATRIX_DIMS + new_col as usize;

            let mut new_state = state.to_vec();
            new_state.swap(index, new_index);

            queue.push_back((new_state, move_count + 1));
        }
    }
}

fn puzzle(initial_state: &[u8], goal_state: &[u8]) {
    let mut found = false;

    // It's BFS because it's a queue, not a stack (DFS).
    let mut queue = VecDeque::from([(initial_state.to_vec(), 0)]);
    let mut visited = HashSet::new();

    let mut outer_node: Option<Vec<u8>> = None;
    let mut outer_moves: usize = 0;
    let mut c = 0;

    while let Some((node, moves)) = queue.pop_front() {
        c += 1;
        if found { break; }
        if visited.contains(&node) { continue; }

        visited.insert(node.clone());

        if is_objective(&node, goal_state) {
            found = true;
            outer_node = Some(node);
            outer_moves = moves;
        } else {
            create_children_and_add_to_queue(&node, moves, &mut queue);
        }
    }

    if found {
        println!("{}", "-".repeat(42));
        println!("initial_state: {:?}", initial_state);
        println!("# VERDICT #");
        println!("Goal state found:");
        println!("{:?}", outer_node);
        println!("Number of moves: {}", outer_moves);
    } else {
        println!("{}", "-".repeat(42));
        println!("initial_state: {:?}", initial_state);
        println!("# VERDICT #");
        println!("No solutions");
        println!("Number of nodes visited: {}", c);
    }
}

fn main() {
    let initial_states = [
        [4, 6, 2, 8, 1, 3, 7, 5, 0],
        [6, 4, 2, 8, 1, 3, 7, 5, 0],
        [1, 2, 3, 4, 5, 6, 7, 8, 0],
        [7, 5, 4, 1, 0, 3, 2, 6, 8],
    ];
    let goal_state = [0, 1, 2, 3, 4, 5, 6, 7, 8];

    for state in initial_states {
        puzzle(&state, &goal_state);
    }
}
