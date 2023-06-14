use crate::chips::overflow::utils::*;

use halo2_proofs::halo2curves::bn256::Fr as Fp;
use halo2_proofs::{circuit::*, plonk::*, poly::Rotation};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct OverflowCheckTConfig<const MAX_BITS: u8, const ACC_COLS: usize> {
    pub a: Column<Advice>,
    pub decomposed_values: Column<Advice>,
    pub range: Column<Fixed>,
    pub selector: Selector,
}

#[derive(Debug, Clone)]
pub struct OverflowChipT<const MAX_BITS: u8, const ACC_COLS: usize> {
    config: OverflowCheckTConfig<MAX_BITS, ACC_COLS>,
}

impl<const MAX_BITS: u8, const ACC_COLS: usize> OverflowChipT<MAX_BITS, ACC_COLS> {
    pub fn construct(config: OverflowCheckTConfig<MAX_BITS, ACC_COLS>) -> Self {
        Self { config }
    }

    pub fn configure(meta: &mut ConstraintSystem<Fp>) -> OverflowCheckTConfig<MAX_BITS, ACC_COLS> {
        let selector = meta.selector();
        let range = meta.fixed_column();
        let a = meta.advice_column();
        let b = meta.advice_column();

        meta.create_gate(
            "equality check between decomposed_value and value",
            |meta| {
                let s_doc = meta.query_selector(selector);

                let value = meta.query_advice(a, Rotation::cur());

                let mut decomposed_value_vec: Vec<Expression<Fp>> = vec![
                    meta.query_advice(b, Rotation::cur()),
                    meta.query_advice(b, Rotation::next()),
                ];

                // decomposed_value_vec.push(
                //     (2..ACC_COLS)
                //         .map(|i: usize| meta.query_advice(b, Rotation(i)))
                //         .collect::<Vec<_>>(),
                // );
                for i in 2..ACC_COLS {
                    decomposed_value_vec.push(meta.query_advice(b, Rotation(i as i32)));
                }

                // multiplier by position of accumulator column
                // e.g. for ACC_COLS = 3, multiplier = [2^(2*MAX_BITS), 2^MAX_BITS, 1]
                let multiplier = |pos: usize| {
                    let mut shift_chunk = Fp::one();
                    for _ in 1..pos {
                        shift_chunk *= Fp::from(1 << MAX_BITS);
                    }
                    Expression::Constant(shift_chunk)
                };

                // We are performing an important calculation here to check for overflow in finite field numbers.
                // A single range table is utilized which applies `1 << 8` to decompose the columns for range checking.
                //
                // Consider the example where ACC_COLS = 3, the decomposed values would be represented as follows:
                //
                // |     | a_0 (value) | b    |
                // |-----|-------------|------|
                // |  x  | 0x1f2f3f    | 0x1f |
                // | x+1 |             | 0x2f |
                // | x+2 |             | 0x3f |
                //
                // Here, each column `a_n` represents a decomposed value.
                // So, decomposed_value_sum would be calculated as b(x) * 2^16 + b(x+1) * 2^8 + b(x+2) * 1.
                //
                // During the iteration process in fold, the following would be the values of `acc`:
                // iteration 0: acc = decomposed_value_vec[1] * ( 1 << 8 ) + decomposed_value_vec[2]
                // iteration 1: acc = decomposed_value_vec[0] * ( 1 << 16 ) + decomposed_value_vec[1] * ( 1 << 8 ) + decomposed_value_vec[2]
                let decomposed_value_sum = (0..=ACC_COLS - 2).fold(
                    // decomposed value at right-most advice columnis is least significant byte
                    decomposed_value_vec[ACC_COLS - 1].clone(),
                    |acc, i| {
                        let cursor = ACC_COLS - i;
                        acc + (decomposed_value_vec[i].clone() * multiplier(cursor))
                    },
                );
                println!("decomposed_value_sum: {:?}", decomposed_value_sum);

                vec![s_doc * (decomposed_value_sum - value)]
            },
        );

        meta.annotate_lookup_any_column(range, || "LOOKUP_MAXBITS_RANGE");

        // [0..ACC_COLS].map(|i| {
        //     meta.lookup_any("range check for MAXBITS", |meta| {
        //         let cell = meta.query_advice(b, Rotation(i as i32));
        //         let range = meta.query_fixed(range, Rotation::cur());
        //         vec![(cell, range)]
        //     });
        // });

        // for i in 2..ACC_COLS {
        //     meta.lookup_any("range check for MAXBITS", |meta| {
        //         // let cell = meta.query_advice(b, Rotation::cur());
        //         let cell = meta.query_advice(b, Rotation(i as i32));
        //         let range = meta.query_fixed(range, Rotation(i as i32));
        //         // TODO: query TableColumn

        //         // let range = meta.query_fixed(range, Rotation::cur());
        //         println!("i: {}, cell: {:?}, range: {:?}", i, cell, range);
        //         vec![(cell, range)]
        //     });
        // }

        let lookup_result_0 = meta.lookup_any("range check within MAXBITS - row 0", |meta| {
            let cell = meta.query_advice(b, Rotation::cur());
            let range = meta.query_fixed(range, Rotation::cur());
            println!("cell: {:?}, range: {:?}", cell, range);
            vec![(cell, range)]
        });
        println!("lookup_result_0: {:?}", lookup_result_0);

        let lookup_result_1 = meta.lookup_any("range check within MAXBITS - row 1", |meta| {
            let cell = meta.query_advice(b, Rotation::next());
            let range = meta.query_fixed(range, Rotation::cur());

            println!("cell_1: {:?}, range: {:?}", cell, range);
            vec![(cell, range)]
        });
        println!("lookup_result_1: {:?}", lookup_result_1);

        let lookup_result_2 = meta.lookup_any("range check within MAXBITS - row 2", |meta| {
            let cell = meta.query_advice(b, Rotation(2));
            let range = meta.query_fixed(range, Rotation::cur());

            println!("cell_2: {:?}, range: {:?}", cell, range);
            vec![(cell, range)]
        });
        println!("lookup_result_2: {:?}", lookup_result_2);

        OverflowCheckTConfig {
            a,
            decomposed_values: b,
            range,
            selector,
        }
    }

    pub fn assign(
        &self,
        mut layouter: impl Layouter<Fp>,
        value: &AssignedCell<Fp, Fp>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "assign decomposed values",
            |mut region| {
                // enable selector
                self.config.selector.enable(&mut region, 0)?;

                // Assign input value to the cell inside the region
                let _ = value.copy_advice(|| "assign value", &mut region, self.config.a, 0);

                // Just used helper function for decomposing. In other halo2 application used functions based on Field.
                let decomposed_values: Vec<Fp> = decompose_bigint_to_ubits(
                    &value_fp_to_big_uint(value.value().map(|x| x.to_owned())),
                    ACC_COLS,
                    MAX_BITS as usize,
                ) as Vec<Fp>;

                println!("decomposed_values: {:?}", decomposed_values);

                // Note that, decomposed result is little edian. So, we need to reverse it.
                for (idx, val) in decomposed_values.iter().rev().enumerate() {
                    let _cell = region.assign_advice(
                        || format!("assign decomposed[{}] row", idx),
                        self.config.decomposed_values,
                        idx,
                        || Value::known(*val),
                    )?;

                    println!("_cell: {:?}", _cell);
                }

                Ok(())
            },
        )
    }

    pub fn load(&self, layouter: &mut impl Layouter<Fp>) -> Result<(), Error> {
        let range = 1 << (MAX_BITS as usize);

        layouter.assign_region(
            || format!("load range check table of {} bits", MAX_BITS),
            |mut region| {
                for i in 0..range {
                    let _cell = region.assign_fixed(
                        || "assign cell in fixed column",
                        self.config.range,
                        i,
                        || Value::known(Fp::from(i as u64)),
                    )?;
                    // println!("load cell: {:?}", _cell);
                }
                Ok(())
            },
        )
    }
}
