#![no_main]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let x: usize = env::read();
    let x_cube = x.pow(3);
    let y = x_cube + x + 5;
    env::commit(&y);
}
