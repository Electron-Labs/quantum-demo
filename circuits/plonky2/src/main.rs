use std::fs;

use plonky2::{
    field::types::Field,
    iop::witness::{PartialWitness, WitnessWrite},
    plonk::{
        circuit_builder::CircuitBuilder,
        circuit_data::CircuitConfig,
        config::{GenericConfig, PoseidonGoldilocksConfig},
    },
    util::serialization::DefaultGateSerializer,
};

fn main() {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_ecc_config());

    let x = builder.add_virtual_target();
    let x_square = builder.mul(x, x);
    let x_cube = builder.mul(x_square, x);
    let y_intermediate = builder.add(x_cube, x);
    let five = builder.constant(<F as Field>::from_canonical_u8(5));
    let y = builder.add(y_intermediate, five);
    builder.register_public_input(y);

    let data = builder.build::<C>();

    let mut pw = PartialWitness::<F>::new();
    pw.set_target(x, F::from_canonical_u8(3));
    let proof = data.prove(pw).unwrap();
    assert!(data.verify(proof.clone()).is_ok());

    let path = "circuits/plonky2/circuit_data";
    fs::create_dir_all(path).unwrap();

    // dump common data
    let common_data_bytes = data.common.to_bytes(&DefaultGateSerializer).unwrap();
    fs::write(format! {"{path}/common_data.bin"}, common_data_bytes).unwrap();

    // dump verifier_only data
    let verifier_only_bytes = data.verifier_only.to_bytes().unwrap();
    fs::write(format! {"{path}/verifier_only.bin"}, verifier_only_bytes).unwrap();

    // dump proof
    let proof_bytes = proof.to_bytes();
    fs::write(format! {"{path}/proof.bin"}, proof_bytes).unwrap();
}
