pragma circom 2.0.0;

/* x**3+x+5 == y */

template Circuit () {
   signal input x;
   signal output y;
   signal x_square <== x * x;
   signal x_cube <== x_square * x;
   y <== x_cube + x + 5;
}

component main = Circuit();