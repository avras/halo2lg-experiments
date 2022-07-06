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
struct MiMC5HashConfig {
    instance: Column<Instance>,
    state: Column<Advice>,
    round_constants: Column<Fixed>,
    s_in_rounds: Selector,
}
struct MiMC5HashChip<F: FieldExt> {
    config: MiMC5HashConfig,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> MiMC5HashChip<F> {
    pub fn construct(config: MiMC5HashConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        instance: Column<Instance>,
        state: Column<Advice>,
        round_constants: Column<Fixed>,
    ) -> MiMC5HashConfig {
        let s_in_rounds = meta.selector();

        meta.enable_equality(instance);
        meta.enable_equality(state);
        meta.enable_constant(round_constants);

        // instance      | state                    | round_constants   | selector
        // message       | x0 = message             |     c0            | 
        // message hash  | x1 = (x0+c0)^5           |     c1            | s_in_rounds
        //               | x2 = (x1+c1)^5           |     c2            | s_in_rounds
        //               | x3 = (x2+c2)^5           |     c3            | s_in_rounds
        //               | x4 = (x3+c3)^5           |     c4            | s_in_rounds
        //               |      :                   |     :             |     :      
        //               |      :                   |     c109          |     :      
        //               | x110 = (x109+key+c109)^5 |                   | s_in_rounds


        meta.create_gate("MiMC5 hash rounds", |meta| {
            let s = meta.query_selector(s_in_rounds);
            let pow_5_expr = |v: Expression<F>| {
                 v.clone() * v.clone() * v.clone() * v.clone() * v
            };
            let prev_state = meta.query_advice(state, Rotation::prev());
            let rc = meta.query_fixed(round_constants, Rotation::prev());
            let current_state = meta.query_advice(state, Rotation::cur());
            vec![
                s.clone()*(current_state - pow_5_expr(prev_state +  rc)),
            ]
        });

        MiMC5HashConfig {
            instance,
            state,
            round_constants,
            s_in_rounds,
        }
    }

    pub fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        initial_value: F,
        round_constants: &Vec<F>,
        num_rounds: usize,
    ) -> Result<(AssignedCell<F,F>, AssignedCell<F,F>), Error> {
        layouter.assign_region(
            || "MiMC5 table",
            |mut region| {

                let msg_cell = 
                region.assign_advice_from_instance(
                    || "message to be hashed",
                    self.config.instance,
                    0,
                    self.config.state,
                    0
                )?;

                let pow_5 = |v: F| { v*v*v*v*v };

                let mut current_state = initial_value;
                let mut state_cell = msg_cell.clone();
                for i in 1..=num_rounds {
                    self.config.s_in_rounds.enable(&mut region, i)?;
                    region.assign_fixed(
                        || format!("round constant {:?}", i),
                        self.config.round_constants,
                        i-1,
                        || Value::known(round_constants[i-1]) // i starts at 1
                    )?;

                    current_state = pow_5(current_state + round_constants[i-1]);
                    
                    state_cell =
                    region.assign_advice(
                        || format!("round {:?} output", i),
                        self.config.state,
                        i,
                        || Value::known(current_state)
                    )?;
                }

                Ok((msg_cell, state_cell))
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
struct MiMC5HashCircuit<F> {
    pub message: F,
    pub num_rounds: usize,
    pub round_constants: Vec<F>,
}

impl <F: FieldExt> Circuit<F> for MiMC5HashCircuit<F> {
    type Config = MiMC5HashConfig;
    type FloorPlanner = SimpleFloorPlanner;
    
    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let instance = meta.instance_column();
        let state = meta.advice_column();
        let round_constants = meta.fixed_column();
        MiMC5HashChip::configure(meta, instance, state, round_constants)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip = MiMC5HashChip::construct(config);

        let (msg, msg_hash) = chip.assign(
            layouter.namespace(|| "entire table"),
            self.message,
            &self.round_constants,
            self.num_rounds,
        )?;

        chip.expose_public(layouter.namespace(|| "message"), msg, 0)?;
        chip.expose_public(layouter.namespace(|| "out"), msg_hash, 1)?;
        // instance      | state 
        // message       |
        // key           |
        // final_output  |

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::mimc_pallas::primitives::mimc5_hash;

    use super::*;
    use halo2_proofs::{dev::MockProver, pasta::Fp};
    use crate::mimc_pallas::pallas_round_constants::{NUM_ROUNDS, ROUND_CONSTANTS};

 
    #[test]
    fn test_mimc5_hash() {
        let k = 7;

        let msg = Fp::from(0);
        let mut output = msg;
        mimc5_hash::<Fp, { NUM_ROUNDS }>(&mut output, ROUND_CONSTANTS);

        let circuit = MiMC5HashCircuit {
            message: msg,
            round_constants: ROUND_CONSTANTS.to_vec(),
            num_rounds: NUM_ROUNDS,
        };

        let public_input = vec![msg, output];

        let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
        prover.assert_satisfied();

    }

    #[cfg(feature = "dev-graph")]
    #[test]
    fn plot_mimc5_hash() {
        use plotters::prelude::*;
        let k = 7;
        let root = BitMapBackend::new("mimc5-hash-layout.png", (1024, 3096)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.titled("MiMC Hash Layout", ("sans-serif", 60)).unwrap();

        let circuit = MiMC5HashCircuit {
            message: Fp::zero(),
            round_constants: ROUND_CONSTANTS.to_vec(),
            num_rounds: NUM_ROUNDS,
        };

        halo2_proofs::dev::CircuitLayout::default()
            .render(k, &circuit, &root)
            .unwrap();
    }
}