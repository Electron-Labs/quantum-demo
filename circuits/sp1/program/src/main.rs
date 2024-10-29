#![no_main]
sp1_zkvm::entrypoint!(main);

fn main() {
    let x = sp1_zkvm::io::read::<usize>();
    let x_cube = x.pow(3);
    let y = x_cube + x + 5;
    sp1_zkvm::io::commit(&y);
}
