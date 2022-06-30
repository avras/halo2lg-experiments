use halo2_proofs::{arithmetic::FieldExt, circuit::*, plonk::*, poly::Rotation};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
struct ACell<F: FieldExt>(AssignedCell<F, F>);

#[derive(Debug, Clone)]
struct CollatzConfig {
    element: Column<Advice>,
    is_even: Column<Advice>,
    selector: Selector,
    instance: Column<Instance>,
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
        element: Column<Advice>,
        is_even: Column<Advice>,
        instance: Column<Instance>,
    ) -> CollatzConfig {
        let selector = meta.selector();

        meta.enable_equality(element);
        meta.enable_equality(is_even);
        meta.enable_equality(instance);

        meta.create_gate("collatz sequence rules", |meta| {
            let s = meta.query_selector(selector);
            let curr_el = meta.query_advice(element, Rotation::cur());
            let next_el = meta.query_advice(element, Rotation::next());
            let even_flag = meta.query_advice(is_even, Rotation::cur());
            let one = Expression::Constant(F::one());
            let two = Expression::Constant(F::from(2));
            let three = Expression::Constant(F::from(3));
            
            let next_el_if_odd = three * curr_el.clone() + one.clone();

            vec![s * (even_flag.clone() * (curr_el - two *next_el.clone()) + (one-even_flag)*(next_el_if_odd-next_el)  )]
        });

        CollatzConfig {
            element,
            is_even,
            selector,
            instance,
        }
    }

    pub fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        initial_value: F,
        nrows: usize,
    ) -> Result<AssignedCell<F, F>, Error> {
        layouter.assign_region(
            || "entire Collatz table",
            |mut region| {
                self.config.selector.enable(&mut region, 0)?;
                let mut is_even_flag = if initial_value.is_even().into() {
                    F::one()
                } else {
                    F::zero()
                };

                region.assign_advice(|| "is_even", self.config.is_even, 0, || Value::known(is_even_flag))?;

                let mut a_cell = region.assign_advice_from_instance(
                    || "1",
                    self.config.instance,
                    0,
                    self.config.element,
                    0,
                )?;

                let mut prev_value = initial_value;
                let mut current_value;
                for row in 1..nrows {
                    if row < nrows - 1 {
                        self.config.selector.enable(&mut region, row)?;
                    }

                    if prev_value.is_even().into() {
                        current_value = prev_value * F::TWO_INV
                    } else {
                        current_value = F::from(3)*prev_value + F::one()
                    };

                    let c_cell = region.assign_advice(
                        || "element",
                        self.config.element,
                        row,
                        || Value::known(current_value)
                    )?;

                    is_even_flag = if current_value.is_even().into() {
                        F::one()
                    } else {
                        F::zero()
                    };

                    region.assign_advice(
                        || "is_even",
                        self.config.is_even,
                        row,
                        || Value::known(is_even_flag)
                    )?;
                    prev_value = current_value;
                    a_cell = c_cell;
                }

                Ok(a_cell)
            },
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
struct MyCircuit<F> {
    pub initial_value: F,
    pub nrows: usize,
}

impl<F: FieldExt> Circuit<F> for MyCircuit<F> {
    type Config = CollatzConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let element = meta.advice_column();
        let is_even = meta.advice_column();
        let instance = meta.instance_column();
        CollatzChip::configure(meta, element, is_even, instance)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip = CollatzChip::construct(config);

        let out_cell = chip.assign(layouter.namespace(|| "entire table"), self.initial_value, self.nrows)?;

        chip.expose_public(layouter.namespace(|| "out"), out_cell, 1)?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::{dev::MockProver, pasta::Fp};

 
    #[test]
    fn test_collatz_one_column() {
        let k = 16;

        let initial_value = Fp::from(100);
        let final_value = Fp::from(1);
        let nrows = 26;

        let circuit = MyCircuit {initial_value, nrows };

        let public_input = vec![initial_value, final_value];

        let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
        prover.assert_satisfied();
        for i in (0..k).rev() {
            println!("Hello {:?}", i);
        }

    }

    #[cfg(feature = "dev-graph")]
    #[test]
    fn plot_fibo1() {
        use plotters::prelude::*;
        let root = BitMapBackend::new("fib1-layout.png", (1024, 3096)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.titled("Fib 1 Layout", ("sans-serif", 60)).unwrap();

        let circuit = MyCircuit::<Fp>(PhantomData);
        halo2_proofs::dev::CircuitLayout::default()
            .render(4, &circuit, &root)
            .unwrap();
    }
}