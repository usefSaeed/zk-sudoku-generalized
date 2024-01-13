pragma circom 2.1.7;

include "helperTemplates.circom";

// Check if both inputs are equal or old input is empty
template Consistent() {
    signal input in_old;
    signal input in_new;
    signal same;
    same <-- in_new - in_old;
    same * in_old === 0;
}

// Checks if input array has distinct n elements
template Distinct(n) {
    signal input in[n];
    component nonEqual[n][n];
    // log(in[0],in[1],in[2],in[3],in[4],in[5],in[6],in[7],in[8]);
    for(var i = 0; i < n; i++){
        for(var j = 0; j < i; j++){
            nonEqual[i][j] = NonEqual();
            nonEqual[i][j].in0 <== in[i];
            nonEqual[i][j].in1 <== in[j];
        }
    }
}

// Enforce that 1 <= in <= n
template OneToN(n,b) {
    signal input in;
    component lowerBound = FitsBits(b);
    component upperBound = FitsBits(b);
    lowerBound.in <== in - 1;
    upperBound.in <== in + 2**b - 1 - n;
}

// Checks n is not zero, is perfect square 
// Checks sqrt_n and log_n are computed correctly
template SudokuSizeCheck(){
    signal input n_test;
    signal input sqrt_n_test;
    signal input log_n_test;
    component nz = NonZero();
    component sc = SqrtChecker();
    component lc = Log2Checker();
    nz.n <== n_test;
    sc.x <== n_test; sc.sqrt_x <== sqrt_n_test;
    lc.n <== n_test; lc.log_n <== log_n_test;
}