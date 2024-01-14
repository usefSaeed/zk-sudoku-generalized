pragma circom 2.1.7;

include "baseTemplates.circom";

template Sudoku(n,sqrt_n,log_n_plus_one) {
    // solution is a 2D array: indices are (row_i, col_i)
    signal input solution[n][n];
    // puzzle is the same, but a zero indicates a blank
    signal input puzzle[n][n];

    component isValidN; 
    component samePuzzle[n][n];
    component distinct[3][n];
    component inRange[n][n];

    //Checking if n belong to {1,4,9,16,..} and inputs are consistant
    isValidN = SudokuSizeCheck();
    isValidN.n_test <== n; isValidN.log_n_test <== log_n_plus_one; isValidN.sqrt_n_test <== sqrt_n;

    //Checking if filled in cells are still the same in solution
    //Checking if all empty cells in puzzle now has a value in solution
    for (var row_i = 0; row_i < n; row_i++) {
        for (var col_i = 0; col_i < n; col_i++) {
            samePuzzle[row_i][col_i] = Consistent();
            samePuzzle[row_i][col_i].in_old <== puzzle[row_i][col_i] ;
            samePuzzle[row_i][col_i].in_new <== solution[row_i][col_i];
        }
    }

    //Checking if each row has distinct n elements
    for (var col_i = 0; col_i < n; col_i++) {
        for (var row_i = 0; row_i < n; row_i++) {
            if (col_i == 0) {
                distinct[1][row_i] = Distinct(n);
            }
            // log("Checking[",row_i,"]","[",col_i,"]");
            inRange[row_i][col_i] = OneToN(n,log_n);
            inRange[row_i][col_i].in <== solution[row_i][col_i];
            distinct[1][row_i].in[col_i] <== solution[row_i][col_i];
        }
    }

    //Checking if each column has distinct n elements
    for (var row_i = 0; row_i < n; row_i++) {
        for (var col_i = 0; col_i < n; col_i++) {
            if (row_i == 0) {
                distinct[0][col_i] = Distinct(n);
            }
            // if (row_i==n-1) log("Checking col",row_i);
            distinct[0][col_i].in[row_i] <== solution[row_i][col_i];
        }
    }


    // Checking if each subgrid has distinct n elements
    var subgrid_i = 0;
    for (var h_section_i = 0; h_section_i < n; h_section_i += sqrt_n){
        for (var v_section_i = 0; v_section_i < n; v_section_i += sqrt_n){
            var filling_i = 0;
            for (var subgrid_row_i = 0; subgrid_row_i < sqrt_n; subgrid_row_i++) {
                for (var subgrid_col_i = 0; subgrid_col_i < sqrt_n; subgrid_col_i++) {
                    if (subgrid_i == 0){
                        distinct[2][filling_i] = Distinct(n);
                    }
                    var row_i = h_section_i + subgrid_row_i;
                    var col_i = v_section_i + subgrid_col_i;
                    // log("Subgrid",subgrid_i,"Box",filling_i,"Row",row_i,"Col",col_i,"=",solution[row_i][col_i]);
                    distinct[2][subgrid_i].in[filling_i] <== solution[row_i][col_i];
                    filling_i += 1;
                }
            }
            subgrid_i += 1;
        }
    }
}

component main {public[puzzle]} = Sudoku(4,2,3);
