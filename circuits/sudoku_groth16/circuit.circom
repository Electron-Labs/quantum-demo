pragma circom 2.0.0;

template IsZero() {
    signal input in;
    signal output out;
    signal inv;
    
    inv <-- in!=0 ? 1/in : 0;
    out <== -in*inv + 1;
}

template SudokuVerifier() {
    // Public input - the puzzle (0 represents empty cells)
    signal input puzzle[81];
    // Private input - the solution
    signal input solution[9][9];
    // Output signal - will be 1 if valid
    signal output is_valid;

    // Temporary signals
    signal row_sums[9];
    signal row_valid[9];
    signal col_sums[9];
    signal col_valid[9];
    signal box_sums[9];
    signal box_valid[9];
    signal match_signals[9][9];
    signal match_sum;
    signal range_valid[9][9];
    signal total_valid;
    
    // Intermediate validation signals
    signal row_col_valid[9];
    signal box_range_valid[9][9];
    signal final_valid[9];
    signal temp_valid[9];
    
    // Pre-declare IsZero components
    component is_empty[9][9];
    for (var i = 0; i < 9; i++) {
        for (var j = 0; j < 9; j++) {
            is_empty[i][j] = IsZero();
        }
    }
    
    // Check rows and range (1-9)
    var temp;
    for (var i = 0; i < 9; i++) {
        temp = 0;
        for (var j = 0; j < 9; j++) {
            temp += solution[i][j];
            
            // Range check: value must be between 1 and 9
            range_valid[i][j] <== (solution[i][j] * (10 - solution[i][j]));
        }
        row_sums[i] <== temp;
        row_valid[i] <== (45 - row_sums[i]) * (row_sums[i] - 45);
    }
    
    // Check columns
    for (var j = 0; j < 9; j++) {
        temp = 0;
        for (var i = 0; i < 9; i++) {
            temp += solution[i][j];
        }
        col_sums[j] <== temp;
        col_valid[j] <== (45 - col_sums[j]) * (col_sums[j] - 45);
    }
    
    // Check 3x3 boxes
    for (var box = 0; box < 9; box++) {
        temp = 0;
        var start_row = (box \ 3) * 3;
        var start_col = (box % 3) * 3;
        for (var i = 0; i < 3; i++) {
            for (var j = 0; j < 3; j++) {
                temp += solution[start_row + i][start_col + j];
            }
        }
        box_sums[box] <== temp;
        box_valid[box] <== (45 - box_sums[box]) * (box_sums[box] - 45);
    }
    
    // Check puzzle matches solution where puzzle is not empty
    temp = 0;
    for (var i = 0; i < 9; i++) {
        for (var j = 0; j < 9; j++) {
            var idx = i * 9 + j;
            is_empty[i][j].in <== puzzle[idx];
            match_signals[i][j] <== (1 - is_empty[i][j].out) * (puzzle[idx] - solution[i][j]);
            temp += match_signals[i][j];
        }
    }
    match_sum <== temp;
    
    // Combine validations in steps to keep constraints quadratic
    // First combine row and column validations
    for (var i = 0; i < 9; i++) {
        row_col_valid[i] <== (1 - row_valid[i]) * (1 - col_valid[i]);
    }
    
    // Then combine box and range validations
    for (var i = 0; i < 9; i++) {
        box_range_valid[i][0] <== (1 - box_valid[i]) * range_valid[i][0];
        for (var j = 1; j < 9; j++) {
            box_range_valid[i][j] <== box_range_valid[i][j-1] * range_valid[i][j];
        }
    }
    
    // Final combination in steps
    // First combine row_col with box_range for each position
    for (var i = 0; i < 9; i++) {
        temp_valid[i] <== row_col_valid[i] * box_range_valid[i][8];
    }
    
    // Then combine results sequentially
    final_valid[0] <== temp_valid[0];
    for (var i = 1; i < 9; i++) {
        final_valid[i] <== final_valid[i-1] * temp_valid[i];
    }
    
    // Output will be 1 if valid (all checks pass)
    is_valid <== final_valid[8] * (1 - match_sum);
}

component main = SudokuVerifier(); 