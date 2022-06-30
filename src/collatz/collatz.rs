use std::marker::PhantomData;

use halo2_proofs::{plonk::*, arithmetic::FieldExt, poly::Rotation, circuit::*};

#[derive(Debug, Clone)]
struct ACell<F: FieldExt>(AssignedCell<F, F>);

#[derive(Debug, Clone)]
struct CollatzConfig<const WIDTH: usize> {
    // Option is to allow initialization with [None; WIDTH] in CollatzCircuit::configure
    pub advice: [Option<Column<Advice>>; WIDTH],
    pub s_all_rows: Selector,
    pub s_last_row: Selector,
    pub s_non_last_row: Selector,
    //pub instance: Column<Instance>,
}

#[derive(Debug, Clone)]
struct CollatzChip<F: FieldExt, const WIDTH: usize> {
    config: CollatzConfig<WIDTH>,
    _marker: PhantomData<F>,
}

impl<F: FieldExt, const WIDTH: usize> CollatzChip<F, WIDTH> {

    pub fn construct(config: CollatzConfig<WIDTH>) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Option<Column<Advice>>; WIDTH],
        //instance: Column<Instance>,
    ) -> CollatzConfig<WIDTH> {
        let s_all_rows = meta.selector();
        let s_last_row = meta.selector();
        let s_non_last_row = meta.selector();

        //meta.enable_equality(instance);
        for column in advice {
            meta.enable_equality(column.unwrap());
        }

        // Constrain all the advice columns to be bits
        meta.create_gate("advice columns are bits", |meta| {
            let s = meta.query_selector(s_all_rows);
            let mut bit;
            let mut constraints = vec![];

            for i in 0..WIDTH {
                bit = meta.query_advice(advice[i].unwrap(), Rotation::cur());
                // s * bit * (1-bit) = 0
                constraints.push(s.clone() * bit.clone() * (Expression::Constant(F::one()) - bit));
            }
            constraints
        });

        // Constrain the last advice row to be equal to 1 (the end of the Collatz sequence)
        meta.create_gate("last advice row equals a 1", |meta| {
            let s = meta.query_selector(s_last_row);
            let mut bit;
            let mut constraints = vec![];

            for i in 0..WIDTH-1 {
                bit = meta.query_advice(advice[i].unwrap(), Rotation::cur());
                // s*bit = 0
                constraints.push(s.clone() * bit);
            }
            bit = meta.query_advice(advice[WIDTH-1].unwrap(), Rotation::cur());
            // s * (1-bit) = 0
            constraints.push(s.clone() * (Expression::Constant(F::one()) - bit));
            constraints
        });

        // For all rows except the last row, constrain the value stored in advice bits
        // to follow the Collatz sequence rules
        // If previous element n is even, current element is n/2
        // If previous element n is odd, current element is 3*n+1
        meta.create_gate("Collatz sequence rule", |meta| {
            let s = meta.query_selector(s_non_last_row);
            let lsb = meta.query_advice(advice[WIDTH-1].unwrap(), Rotation::cur()); // least significant bit
            let mut bit;
            let mut current_element = Expression::Constant(F::zero());
            let mut next_element = Expression::Constant(F::zero());
            let mut coeff = Expression::Constant(F::one());
            let one = Expression::Constant(F::one());
            let two = Expression::Constant(F::from(2));
            let three = Expression::Constant(F::from(3));

            // Calculate integer value of current advice row
            for i in (0..WIDTH).rev() {
                bit = meta.query_advice(advice[i].unwrap(), Rotation::cur());
                current_element = current_element + bit * coeff.clone();
                coeff = coeff * Expression::Constant(F::from(2));
            }
            
            // Calculate integer value of next advice row
            coeff = Expression::Constant(F::one());
            for i in (0..WIDTH).rev() {
                bit = meta.query_advice(advice[i].unwrap(), Rotation::next());
                next_element = next_element + bit * coeff.clone();
                coeff = coeff * Expression::Constant(F::from(2));
            }

            let next_element_if_odd = three * current_element.clone() + one.clone();
            // s* [ lsb * (a_{i+1} - a_i) + (1-lsb) * (a_i - 2*a_{i+1})] = 0
            vec![s*(lsb.clone() * (next_element_if_odd - next_element.clone()) + (one-lsb) * (current_element - two * next_element))]
        });


        CollatzConfig {
            advice,
            s_all_rows,
            s_last_row,
            s_non_last_row,
            //Instance,
        }
    }

    pub fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        initial_value: F,
        nrows: usize,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "Entire Collatz sequence bits",
            |mut region| {

                let mut current_value = initial_value;
                let mut next_value;

                for row in 0..nrows {
                    self.config.s_all_rows.enable(&mut region, row)?;
                    if row < nrows-1 {
                        self.config.s_non_last_row.enable(&mut region, row)?;
                    }
                    if row == nrows-1 {
                        self.config.s_last_row.enable(&mut region, row)?;
                    }

                    if current_value.is_even().into() {
                        next_value = current_value * F::TWO_INV;
                    } else {
                        next_value = F::from(3)*current_value + F::one();
                    }

                    let mut bit;

                    for i in (0..WIDTH).rev() {
                        if current_value.is_even().into() {
                            bit = F::zero();
                            current_value = current_value * F::TWO_INV;
                        } else {
                            bit = F::one();
                            current_value = (current_value - F::one()) * F::TWO_INV;
                        }
                        
                        let _a =
                        region
                        .assign_advice(|| format!("row {:?} bit {:?}", row, i), self.config.advice[i].unwrap(), row, || Value::known(bit))?;
                    }
                    current_value = next_value;
                }
                Ok(())
            }
        )
    }

}

#[derive(Default)]
struct CollatzCircuit<F: FieldExt, const WIDTH: usize> {
    pub initial_value: F,
    pub nrows: usize,
}

impl <F: FieldExt, const WIDTH: usize> Circuit<F> for CollatzCircuit<F, WIDTH> {
    type Config = CollatzConfig<WIDTH>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let mut advice: [Option<Column<Advice>>; WIDTH] = [None; WIDTH];
        for i in 0..WIDTH {
            advice[i] = Some(meta.advice_column());
        }
        //let instance = meta.instance_column();
        CollatzChip::configure(meta, advice, /*instance*/)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip = CollatzChip::construct(config);
        chip.assign(layouter.namespace(|| "entire table"), self.initial_value, self.nrows)?;

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::{dev::MockProver, pasta::Fp};

 
    #[test]
    fn test_collatz() {
        let k = 16;
        const WIDTH: usize = 6;

        let circuit: CollatzCircuit<Fp, WIDTH> = CollatzCircuit {
            initial_value: Fp::from(52),
            nrows: 12,
        };

        let prover = MockProver::run(k, &circuit, vec![]).unwrap();
        prover.assert_satisfied();
    }

    #[cfg(feature = "dev-graph")]
    #[test]
    fn plot_collatz() {
        use plotters::prelude::*;
        let root = BitMapBackend::new("collatz-layout.png", (1024, 3096)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.titled("Collatz Layout", ("sans-serif", 60)).unwrap();

        const WIDTH: usize = 6;
        let circuit = CollatzCircuit::<Fp, WIDTH>{
            initial_value: Fp::from(52),
            nrows: 12
        };
        halo2_proofs::dev::CircuitLayout::default()
            .render(4, &circuit, &root)
            .unwrap();
    }
}

