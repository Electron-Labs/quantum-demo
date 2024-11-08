use halo2_base::halo2_proofs;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::bn256::{Bn256, Fr, G1Affine},
    plonk::{
        keygen_pk, keygen_vk, Advice, Assigned, Circuit, Column, ConstraintSystem, Error, Fixed,
        Instance, ProvingKey,
    },
    poly::{kzg::commitment::ParamsKZG, Rotation},
};
use rand::{rngs::OsRng, RngCore};
use snark_verifier::system::halo2::{compile, Config};
use snark_verifier_sdk::evm::gen_evm_proof_shplonk;
use std::{fs::File, io::BufWriter};

#[derive(Clone, Copy)]
struct StandardPlonkConfig {
    a: Column<Advice>,
    b: Column<Advice>,
    c: Column<Advice>,
    q_a: Column<Fixed>,
    q_b: Column<Fixed>,
    q_c: Column<Fixed>,
    q_ab: Column<Fixed>,
    constant: Column<Fixed>,
    #[allow(dead_code)]
    instance: Column<Instance>,
}

impl StandardPlonkConfig {
    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self {
        let [a, b, c] = [(); 3].map(|_| meta.advice_column());
        let [q_a, q_b, q_c, q_ab, constant] = [(); 5].map(|_| meta.fixed_column());
        let instance = meta.instance_column();

        [a, b, c].map(|column| meta.enable_equality(column));

        meta.create_gate(
            "q_a·a + q_b·b + q_c·c + q_ab·a·b + constant + instance = 0",
            |meta| {
                let [a, b, c] = [a, b, c].map(|column| meta.query_advice(column, Rotation::cur()));
                let [q_a, q_b, q_c, q_ab, constant] = [q_a, q_b, q_c, q_ab, constant]
                    .map(|column| meta.query_fixed(column, Rotation::cur()));
                let instance = meta.query_instance(instance, Rotation::cur());
                Some(
                    q_a * a.clone()
                        + q_b * b.clone()
                        + q_c * c
                        + q_ab * a * b
                        + constant
                        + instance,
                )
            },
        );

        StandardPlonkConfig {
            a,
            b,
            c,
            q_a,
            q_b,
            q_c,
            q_ab,
            constant,
            instance,
        }
    }
}

#[derive(Clone, Default)]
struct StandardPlonk(Fr);

impl StandardPlonk {
    fn rand<R: RngCore>(mut rng: R) -> Self {
        Self(Fr::from(rng.next_u32() as u64))
    }

    fn num_instance() -> Vec<usize> {
        vec![1]
    }

    fn instances(&self) -> Vec<Vec<Fr>> {
        vec![vec![self.0]]
    }
}

impl Circuit<Fr> for StandardPlonk {
    type Config = StandardPlonkConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        meta.set_minimum_degree(4);
        StandardPlonkConfig::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fr>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "",
            |mut region| {
                #[cfg(feature = "halo2-pse")]
                {
                    region.assign_advice(|| "", config.a, 0, || Value::known(self.0))?;
                    region.assign_fixed(|| "", config.q_a, 0, || Value::known(-Fr::one()))?;

                    region.assign_advice(|| "", config.a, 1, || Value::known(-Fr::from(5u64)))?;
                    for (idx, column) in (1..).zip([
                        config.q_a,
                        config.q_b,
                        config.q_c,
                        config.q_ab,
                        config.constant,
                    ]) {
                        region.assign_fixed(
                            || "",
                            column,
                            1,
                            || Value::known(Fr::from(idx as u64)),
                        )?;
                    }

                    let a = region.assign_advice(|| "", config.a, 2, || Value::known(Fr::one()))?;
                    a.copy_advice(|| "", &mut region, config.b, 3)?;
                    a.copy_advice(|| "", &mut region, config.c, 4)?;
                }
                #[cfg(feature = "halo2-axiom")]
                {
                    region.assign_advice(config.a, 0, Value::known(Assigned::Trivial(self.0)));
                    region.assign_fixed(config.q_a, 0, Assigned::Trivial(-Fr::one()));

                    region.assign_advice(
                        config.a,
                        1,
                        Value::known(Assigned::Trivial(-Fr::from(5u64))),
                    );
                    for (idx, column) in (1..).zip([
                        config.q_a,
                        config.q_b,
                        config.q_c,
                        config.q_ab,
                        config.constant,
                    ]) {
                        region.assign_fixed(column, 1, Assigned::Trivial(Fr::from(idx as u64)));
                    }

                    let a = region.assign_advice(
                        config.a,
                        2,
                        Value::known(Assigned::Trivial(Fr::one())),
                    );
                    a.copy_advice(&mut region, config.b, 3);
                    a.copy_advice(&mut region, config.c, 4);
                }
                Ok(())
            },
        )
    }
}

fn gen_srs(k: u32) -> ParamsKZG<Bn256> {
    ParamsKZG::<Bn256>::setup(k, OsRng)
}

fn gen_pk<C: Circuit<Fr>>(params: &ParamsKZG<Bn256>, circuit: &C) -> ProvingKey<G1Affine> {
    let vk = keygen_vk(params, circuit).unwrap();
    keygen_pk(params, vk, circuit).unwrap()
}

fn main() {
    let params = gen_srs(8);

    let circuit = StandardPlonk::rand(OsRng);
    let pk = gen_pk(&params, &circuit);
    let proof = gen_evm_proof_shplonk(&params, &pk, circuit.clone(), circuit.instances());

    let protocol = compile(
        &params,
        pk.get_vk(),
        Config::kzg()
            .with_num_instance(StandardPlonk::num_instance())
            .with_accumulator_indices(None),
    );

    {
        let dump_path = "circuit_data";

        // write sg2
        let file = File::create(format!("{dump_path}/sg2.json")).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &params.s_g2()).unwrap();

        // write proof
        std::fs::write(format!("{dump_path}/proof.bin"), &proof).unwrap();

        // write protocol
        let file = File::create(format!("{dump_path}/protocol.json")).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &protocol).unwrap();

        // write instances
        let file = File::create(format!("{dump_path}/instances.json")).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &circuit.instances()).unwrap();
    };
}
