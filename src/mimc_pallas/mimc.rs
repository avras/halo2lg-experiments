use std::marker::PhantomData;


use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{
        Column, Advice, Fixed, Selector, ConstraintSystem, Instance, Expression, Error, Circuit
    },
    poly::Rotation,
    circuit::{
        Layouter, AssignedCell, Value, SimpleFloorPlanner
    },
};



#[derive(Debug, Clone)]
struct MiMC5Config {
    instance: Column<Instance>,
    state: Column<Advice>,
    key_column: Column<Advice>,
    round_constants: Column<Fixed>,
    s_in_rounds: Selector,
    s_post_rounds: Selector,
}
struct MiMC5Chip<F: FieldExt> {
    config: MiMC5Config,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> MiMC5Chip<F> {
    pub fn construct(config: MiMC5Config) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        instance: Column<Instance>,
        state: Column<Advice>,
        key_column: Column<Advice>,
        round_constants: Column<Fixed>,
    ) -> MiMC5Config {
        let s_in_rounds = meta.selector();
        let s_post_rounds = meta.selector();

        meta.enable_equality(instance);
        meta.enable_equality(state);
        meta.enable_equality(key_column);
        meta.enable_constant(round_constants);

        // instance      | state                    | key_column   | round_constants   | selector
        // message       | x0 = message             |  key         |     c0            | 
        // final_output  | x1 = (x0+key+c0)^5       |  key         |     c1            | s_in_rounds
        //               | x2 = (x1+key+c1)^5       |  key         |     c2            | s_in_rounds
        //               | x3 = (x2+key+c2)^5       |  key         |     c3            | s_in_rounds
        //               | x4 = (x3+key+c3)^5       |  key         |     c4            | s_in_rounds
        //               |      :                   |  :           |     :             |     :      
        //               |      :                   |  :           |     c109          |     :      
        //               | x110 = (x109+key+c109)^5 |  key         |                   | s_in_rounds
        //               | x110 + key               |              |                   | s_post_rounds


        meta.create_gate("MiMC5 rounds", |meta| {
            let s = meta.query_selector(s_in_rounds);
            let pow_5_expr = |v: Expression<F>| {
                 v.clone() * v.clone() * v.clone() * v.clone() * v
            };
            let prev_state = meta.query_advice(state, Rotation::prev());
            let key = meta.query_advice(key_column, Rotation::cur());
            let prev_key = meta.query_advice(key_column, Rotation::cur());
            let rc = meta.query_fixed(round_constants, Rotation::prev());
            let current_state = meta.query_advice(state, Rotation::cur());
            vec![
                s.clone()*(current_state - pow_5_expr(prev_state + key.clone() + rc)),
                s*(prev_key-key)    // Ensure that the keys remain the same from one row to the next
            ]
        });

        meta.create_gate("post rounds key addition", |meta| {
            let s = meta.query_selector(s_post_rounds);
            let prev_state = meta.query_advice(state, Rotation::prev());
            let key = meta.query_advice(key_column, Rotation::prev()); // Using the key from the previous row
            let current_state = meta.query_advice(state, Rotation::cur());
            vec![s*(current_state - (prev_state + key))]
        });

        MiMC5Config {
            instance,
            state,
            key_column,
            round_constants,
            s_in_rounds,
            s_post_rounds 
        }
    }

    pub fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        message: F,
        key: F,
        round_constants: &Vec<F>,
        num_rounds: usize,
    ) -> Result<(AssignedCell<F,F>, AssignedCell<F,F>), Error> {
        layouter.assign_region(
            || "MiMC5 table",
            |mut region| {
                self.config.s_post_rounds.enable(&mut region, num_rounds+1)?;

                let msg_cell = 
                region.assign_advice_from_instance(
                    || "message to be encrypted",
                    self.config.instance,
                    0,
                    self.config.state,
                    0
                )?;

                region.assign_advice(
                    || format!("key in row 0"),
                    self.config.key_column,
                    0,
                    || Value::known(key)
                )?;


                let pow_5 = |v: F| { v*v*v*v*v };

                let mut current_state = message;
                for i in 1..=num_rounds {
                    self.config.s_in_rounds.enable(&mut region, i)?;
                    region.assign_fixed(
                        || format!("round constant {:?}", i),
                        self.config.round_constants,
                        i-1,
                        || Value::known(round_constants[i-1]) // i starts at 1
                    )?;

                    region.assign_advice(
                        || format!("key in row {:?} ", i),
                        self.config.key_column,
                        i,
                        || Value::known(key)
                    )?;

                    current_state = pow_5(current_state + key + round_constants[i-1]);
                    region.assign_advice(
                        || format!("round {:?} output", i),
                        self.config.state,
                        i,
                        || Value::known(current_state)
                    )?;
                }

                current_state = current_state + key;
                let final_state =
                region.assign_advice(
                    || "final state",
                    self.config.state,
                    num_rounds+1,
                    || Value::known(current_state)
                )?;
                Ok((msg_cell, final_state))
            }
        )
    }

    pub fn expose_public(
        &self,
        mut layouter: impl Layouter<F>,
        cell: AssignedCell<F, F>,
        row: usize,
    ) -> Result<(), Error> {
        layouter.constrain_instance(cell.cell(), self.config.instance, row)
    }
}

#[derive(Default)]
struct MiMC5Circuit<F> {
    pub message: F,
    pub key: F,
    pub num_rounds: usize,
    pub round_constants: Vec<F>,
}

impl <F: FieldExt> Circuit<F> for MiMC5Circuit<F> {
    type Config = MiMC5Config;
    type FloorPlanner = SimpleFloorPlanner;
    
    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let instance = meta.instance_column();
        let state = meta.advice_column();
        let key_column = meta.advice_column();
        let round_constants = meta.fixed_column();
        MiMC5Chip::configure(meta, instance, state, key_column, round_constants)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip = MiMC5Chip::construct(config);

        let (msg, final_output) = chip.assign(
            layouter.namespace(|| "entire table"),
            self.message,
            self.key,
            &self.round_constants,
            self.num_rounds,
        )?;

        chip.expose_public(layouter.namespace(|| "message"), msg, 0)?;
        chip.expose_public(layouter.namespace(|| "out"), final_output, 1)?;
        // instance      | state 
        // message       |
        // final_output  |

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::mimc_pallas::primitives::mimc5_encrypt;

    use super::*;
    use halo2_proofs::{dev::MockProver, pasta::Fp};
    use crate::mimc_pallas::pallas_round_constants::{NUM_ROUNDS, ROUND_CONSTANTS};

 
    #[test]
    fn test_mimc5_cipher() {
        let k = 7;

        let msg = Fp::from(0);
        let key = Fp::from(0);
        let mut output = msg;
        mimc5_encrypt::<Fp, { NUM_ROUNDS }>(&mut output, key, ROUND_CONSTANTS);

        let circuit = MiMC5Circuit {
            message: msg,
            key,
            round_constants: ROUND_CONSTANTS.to_vec(),
            num_rounds: NUM_ROUNDS,
        };

        let public_input = vec![msg, output];

        let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
        prover.assert_satisfied();

    }

    #[cfg(feature = "dev-graph")]
    #[test]
    fn plot_mimc5_cipher() {
        use plotters::prelude::*;
        let k = 7;
        let root = BitMapBackend::new("mimc5-cipher-layout.png", (1024, 3096)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.titled("MiMC5 Cipher Layout", ("sans-serif", 60)).unwrap();

        let circuit = MiMC5Circuit {
            message: Fp::zero(),
            key: Fp::zero(),
            round_constants: ROUND_CONSTANTS.to_vec(),
            num_rounds: NUM_ROUNDS,
        };

        halo2_proofs::dev::CircuitLayout::default()
            .render(k, &circuit, &root)
            .unwrap();
    }
}