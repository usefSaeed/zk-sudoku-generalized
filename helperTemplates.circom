pragma circom 2.1.7;

// Checks if in fits in b bits
template FitsBits(b){
    signal input in;
    signal bits[b];
    var bitsum = 0;
    for (var i = 0; i < b; i++) {
        bits[i] <-- (in >> i) & 1;
        bits[i] * (bits[i] - 1) === 0;
        bitsum = bitsum + 2 ** i * bits[i];
    }
    bitsum === in;
}

// Checks if in0 and in1 are not equal
template NonEqual(){
    signal input in0;
    signal input in1;
    signal inv;
    signal one <-- 1;
    inv <-- 1/ (in0 - in1);
    inv * (in0 - in1) === one;
}

// Checks if n is not zero
template NonZero () {
    signal input n;
    signal inv;
    inv <-- 1 / n;
    1 === n * inv;
}

// Checks if sqrt_n is sqrt(n) is not zero
template SqrtChecker (){
    signal input x;
    signal input sqrt_x;
    sqrt_x*sqrt_x === x;
}

// Checks if log_n is log2(n) is not zero
template Log2Checker(){
    signal input n;
    signal input log_n;
    signal out;
    var n_temp = n;
    var nBits = 0;
    while (n_temp >= 1){
        n_temp \= 2;
        nBits++;
    }
    out <-- nBits;
    log("lol ",out);
    out === log_n + log_n*0;
}
