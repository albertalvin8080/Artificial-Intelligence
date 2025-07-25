const BOARD_SIZE: usize = 8;

/// Checks if the current tile (row, col) is not attacked by any other queen inside `state`.
fn is_safe(state: &[usize], row: usize, col: usize) -> bool {
    // exclusive
    for r in 0..row {
        let c = state[r];
        // >> if the cols are the same
        // >> if the diagonals are the same
        // NOTE: The rows will NEVER be the same since they're represented as each element in the array.
        if c == col || (c as isize - col as isize).abs() == (r as isize - row as isize).abs() {
            return false;
        }
    }
    true
}

fn validate_initial_state(initial_state: &[usize]) {
    for (i, &col) in initial_state.iter().enumerate() {
        if !is_safe(initial_state, i, col) {
            panic!("Invalid initial state: {:?}", initial_state);
        }
    }
}

fn print_board(state: &[usize]) {
    print!("  ");
    for n in 0..BOARD_SIZE {
        print!("{} ", n);
    }
    println!();

    for (row, &col) in state.iter().enumerate() {
        let mut line = ['.'; BOARD_SIZE];
        line[col] = 'Q';
        print!("{} ", row);
        for ch in line {
            print!("{} ", ch);
        }
        println!();
    }
}

fn eight_queens(initial_state: &[usize]) {
    validate_initial_state(initial_state);

    let mut found = false;
    let mut stack = vec![initial_state.to_vec()];
    let mut nodes_created = 0;

    let mut solution: Vec<Vec<usize>> = Vec::new();

    while let Some(state) = stack.pop() {
        nodes_created += 1;
        let row = state.len();

        // NOTE: reverse for left-to-right
        for col in (0..BOARD_SIZE).rev() {
            if is_safe(state.as_slice(), row, col) {
                let mut new_state = state.clone();
                new_state.push(col);
                stack.push(new_state);
            }
        }

        if row == BOARD_SIZE {
            found = true;
            solution.push(state.clone());
            break; // found one solution
        }
    }

    println!("{}", "-".repeat(30));
    println!("initial_state: {:?}", initial_state);
    println!("# VEREDICT #");

    println!("Total nodes created: {}", nodes_created);
    if found {
        println!("Valid 8-Queens solution found:");
        print_board(solution[0].as_slice());
    } else {
        println!("No solution found.");
    }
}

fn main() {
    // NOTE: Each element in the array represents a ROW.
    // NOTE: The number represents a COLUMN.
    eight_queens(&[]);          // no queens
    eight_queens(&[4]);         // one queen (0x4)
    eight_queens(&[0, 5]);      // two queens (0x0, 1x5)
    eight_queens(&[1, 5, 7]);   // three queens (0x1, 1x5, 2x7)
    // eight_queens(&[1, 4, 7]);   // three queens (1x1, 1x4, 2x7) NO SOLUTIONS
}
