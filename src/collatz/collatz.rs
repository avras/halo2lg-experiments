use std::marker::PhantomData;

use halo2_proofs::{plonk::*, arithmetic::FieldExt, poly::Rotation, circuit::*};

#[derive(Debug, Clone)]
struct ACell<F: FieldExt>(AssignedCell<F, F>);

const WIDTH: usize = 6;

#[derive(Debug, Clone)]
struct CollatzConfig {
    pub advice0: Column<Advice>,
    pub advice1: Column<Advice>,
    pub advice2: Column<Advice>,
    pub advice3: Column<Advice>,
    pub advice4: Column<Advice>,
    pub advice5: Column<Advice>,
    pub s_all_rows: Selector,
    pub s_last_row: Selector,
    pub s_non_last_row: Selector,
}

#[derive(Debug, Clone)]
struct CollatzChip<F: FieldExt> {
    config: CollatzConfig,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> CollatzChip<F> {

    pub fn construct(config: CollatzConfig) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        advice0: Column<Advice>,
        advice1: Column<Advice>,
        advice2: Column<Advice>,
        advice3: Column<Advice>,
        advice4: Column<Advice>,
        advice5: Column<Advice>,
    ) -> CollatzConfig {
        let s_all_rows = meta.selector();
        let s_last_row = meta.selector();
        let s_non_last_row = meta.selector();

        meta.enable_equality(advice0);
        meta.enable_equality(advice1);
        meta.enable_equality(advice2);
        meta.enable_equality(advice3);
        meta.enable_equality(advice4);
        meta.enable_equality(advice5);

//        // Constrain first row to be bits
//        // TODO: Remove this block
//        meta.create_gate("last row values are bits", |meta| {
//            let s = meta.query_selector(s_last_row);
//            let bit0 = meta.query_advice(advice0, Rotation::cur());
//            vec![s * bit0.clone() * (Expression::Constant(F::one()) - bit0)]
//        });
//
        // Constrain all the advice columns to be bits
        let advice_cols = [advice0, advice1, advice2, advice3, advice4, advice5];
        meta.create_gate("advice columns are bits", |meta| {
            let s = meta.query_selector(s_all_rows);
            let mut bit;
            let mut constraints = vec![];

            for i in 0..WIDTH {
                bit = meta.query_advice(advice_cols[i], Rotation::cur());
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
                bit = meta.query_advice(advice_cols[i], Rotation::cur());
                // s*bit = 0
                constraints.push(s.clone() * bit);
            }
            bit = meta.query_advice(advice_cols[WIDTH-1], Rotation::cur());
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
            let lsb = meta.query_advice(advice_cols[WIDTH-1], Rotation::cur()); // least significant bit
            let mut bit;
            let mut current_element = Expression::Constant(F::zero());
            let mut next_element = Expression::Constant(F::zero());
            let mut coeff = Expression::Constant(F::one());
            let one = Expression::Constant(F::one());
            let two = Expression::Constant(F::from(2));
            let three = Expression::Constant(F::from(3));

            // Calculate integer value of current advice row
            for i in (0..WIDTH).rev() {
                bit = meta.query_advice(advice_cols[i], Rotation::cur());
                current_element = current_element + bit * coeff.clone();
                coeff = coeff * Expression::Constant(F::from(2));
            }
            
            // Calculate integer value of next advice row
            coeff = Expression::Constant(F::one());
            for i in (0..WIDTH).rev() {
                bit = meta.query_advice(advice_cols[i], Rotation::next());
                next_element = next_element + bit * coeff.clone();
                coeff = coeff * Expression::Constant(F::from(2));
            }

            let next_element_if_odd = three * current_element.clone() + one.clone();
            // s* [ lsb * (a_{i+1} - a_i) + (1-lsb) * (a_i - 2*a_{i+1})] = 0
            vec![s*(lsb.clone() * (next_element_if_odd - next_element.clone()) + (one-lsb) * (current_element - two * next_element))]
        });


        CollatzConfig {
            advice0,
            advice1,
            advice2,
            advice3,
            advice4,
            advice5,
            s_all_rows,
            s_last_row,
            s_non_last_row,
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


                    let advice_cols = [
                        self.config.advice0,
                        self.config.advice1,
                        self.config.advice2,
                        self.config.advice3,
                        self.config.advice4,
                        self.config.advice5,
                    ];
                    let mut bit;
                    for i in (0..WIDTH).rev() {
                        if current_value.is_even().into() {
                            bit = F::zero();
                            current_value = current_value * F::TWO_INV;
                        } else {
                            bit = F::one();
                            current_value = (current_value - F::one()) * F::TWO_INV;
                        }
                        
                        region
                        .assign_advice(|| format!("row {:?} bit {:?}", row, i), advice_cols[i], row, || Value::known(bit))?;
                    }
                    current_value = next_value;
                }
                Ok(())
            }
        )
    }

}

#[derive(Default)]
struct CollatzCircuit<F: FieldExt> {
    pub initial_value: F,
    pub nrows: usize,
}

impl <F: FieldExt> Circuit<F> for CollatzCircuit<F> {
    type Config = CollatzConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advice0 = meta.advice_column();
        let advice1 = meta.advice_column();
        let advice2 = meta.advice_column();
        let advice3 = meta.advice_column();
        let advice4 = meta.advice_column();
        let advice5 = meta.advice_column();
        //let instance = meta.instance_column();
        CollatzChip::configure(meta, advice0, advice1, advice2, advice3, advice4, advice5)
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
    fn test_collatz6() {
        let k = 16;

        let circuit: CollatzCircuit<Fp> = CollatzCircuit {
            initial_value: Fp::from(52),
            nrows: 12,
        };

        let prover = MockProver::run(k, &circuit, vec![]).unwrap();
        prover.assert_satisfied();
    }
}

