// Patterned after [halo2wrong ECDSA](https://github.com/privacy-scaling-explorations/halo2wrong/blob/master/ecdsa/src/ecdsa.rs)
use ecc::integer::{IntegerInstructions, Range};
use ecc::maingate::{MainGate, RangeChip, RangeInstructions, RegionCtx};
use ecc::GeneralEccChip;
use ecdsa::ecdsa::{AssignedEcdsaSig, AssignedPublicKey, EcdsaChip, EcdsaConfig};
use halo2_proofs::halo2curves::{
    bn256::Fr as Fp,
    group::{Curve, Group},
    secp256k1::Secp256k1Affine as Secp256k1,
    CurveAffine,
};
use halo2_proofs::{circuit::*, plonk::*};
use rand::rngs::OsRng;

const BIT_LEN_LIMB: usize = 68;
const NUMBER_OF_LIMBS: usize = 4;

#[derive(Debug, Clone)]
pub struct EcdsaConfigWithInstance {
    pub ecdsa_config: EcdsaConfig,
}

impl EcdsaConfigWithInstance {
    pub fn expose_limbs_to_public(
        &self,
        mut layouter: impl Layouter<Fp>,
        pk_x_limbs: Vec<AssignedCell<Fp, Fp>>,
        pk_y_limbs: Vec<AssignedCell<Fp, Fp>>,
        x_row_start: usize,
        y_row_start: usize,
    ) -> Result<(), Error> {
        // loop over pk_x_limbs and pk_y_limbs and expose them to instance column
        for i in 0..4 {
            layouter.constrain_instance(
                pk_x_limbs[i].cell(),
                self.ecdsa_config.main_gate_config.instance,
                x_row_start + i,
            )?;
            layouter.constrain_instance(
                pk_y_limbs[i].cell(),
                self.ecdsa_config.main_gate_config.instance,
                y_row_start + i,
            )?;
        }

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct EcdsaVerifyCircuit {
    pub public_key: Value<Secp256k1>,
    pub signature: Value<(
        <Secp256k1 as CurveAffine>::ScalarExt,
        <Secp256k1 as CurveAffine>::ScalarExt,
    )>,
    pub msg_hash: Value<<Secp256k1 as CurveAffine>::ScalarExt>,

    pub aux_generator: Secp256k1,
    pub window_size: usize,
}

impl EcdsaVerifyCircuit {
    pub fn init(
        public_key: Secp256k1,
        r: <Secp256k1 as CurveAffine>::ScalarExt,
        s: <Secp256k1 as CurveAffine>::ScalarExt,
        msg_hash: <Secp256k1 as CurveAffine>::ScalarExt,
    ) -> Self {
        let aux_generator = <Secp256k1 as CurveAffine>::CurveExt::random(OsRng).to_affine();

        Self {
            public_key: Value::known(public_key),
            signature: Value::known((r, s)),
            msg_hash: Value::known(msg_hash),
            aux_generator,
            window_size: 4,
        }
    }
}

impl Circuit<Fp> for EcdsaVerifyCircuit {
    type Config = EcdsaConfigWithInstance;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let (rns_base, rns_scalar) =
            GeneralEccChip::<Secp256k1, Fp, NUMBER_OF_LIMBS, BIT_LEN_LIMB>::rns();
        let main_gate_config = MainGate::<Fp>::configure(meta);
        let mut overflow_bit_lens: Vec<usize> = vec![];
        overflow_bit_lens.extend(rns_base.overflow_lengths());
        overflow_bit_lens.extend(rns_scalar.overflow_lengths());
        let composition_bit_lens = vec![BIT_LEN_LIMB / NUMBER_OF_LIMBS];

        let range_config = RangeChip::<Fp>::configure(
            meta,
            &main_gate_config,
            composition_bit_lens,
            overflow_bit_lens,
        );

        let ecdsa_config = EcdsaConfig::new(range_config, main_gate_config);

        EcdsaConfigWithInstance { ecdsa_config }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        let mut ecc_chip = GeneralEccChip::<Secp256k1, Fp, NUMBER_OF_LIMBS, BIT_LEN_LIMB>::new(
            config.ecdsa_config.ecc_chip_config(),
        );

        layouter.assign_region(
            || "assign aux values",
            |region| {
                let offset = 0;
                let ctx = &mut RegionCtx::new(region, offset);

                ecc_chip.assign_aux_generator(ctx, Value::known(self.aux_generator))?;
                ecc_chip.assign_aux(ctx, self.window_size, 2)?;
                Ok(())
            },
        )?;

        let ecdsa_chip = EcdsaChip::new(ecc_chip.clone());
        let scalar_chip = ecc_chip.scalar_field_chip();

        let (pk_x_limbs, pk_y_limbs) = layouter.assign_region(
            || "ecdsa verify region",
            |region| {
                let offset = 0;
                let ctx = &mut RegionCtx::new(region, offset);

                let r = self.signature.map(|signature| signature.0);
                let s = self.signature.map(|signature| signature.1);
                let integer_r = ecc_chip.new_unassigned_scalar(r);
                let integer_s = ecc_chip.new_unassigned_scalar(s);
                let msg_hash = ecc_chip.new_unassigned_scalar(self.msg_hash);

                let r_assigned = scalar_chip.assign_integer(ctx, integer_r, Range::Remainder)?;
                let s_assigned = scalar_chip.assign_integer(ctx, integer_s, Range::Remainder)?;
                let sig = AssignedEcdsaSig {
                    r: r_assigned,
                    s: s_assigned,
                };

                let pk_in_circuit = ecc_chip.assign_point(ctx, self.public_key)?;

                let x_clone = pk_in_circuit.x();
                let y_clone = pk_in_circuit.y();

                let pk_x_limbs: Vec<AssignedCell<Fp, Fp>> = x_clone
                    .limbs()
                    .iter()
                    .map(|limb| limb.as_ref().clone())
                    .collect();

                let pk_y_limbs: Vec<AssignedCell<Fp, Fp>> = y_clone
                    .limbs()
                    .iter()
                    .map(|limb| limb.as_ref().clone())
                    .collect();

                let pk_assigned = AssignedPublicKey {
                    point: pk_in_circuit,
                };
                let msg_hash = scalar_chip.assign_integer(ctx, msg_hash, Range::Remainder)?;
                ecdsa_chip.verify(ctx, &sig, &pk_assigned, &msg_hash)?;

                Ok((pk_x_limbs, pk_y_limbs))
            },
        )?;

        config.expose_limbs_to_public(
            layouter.namespace(|| "expose pub key to public"),
            pk_x_limbs,
            pk_y_limbs,
            0,
            4,
        )?;

        let range_chip = RangeChip::<Fp>::new(config.ecdsa_config.range_config);
        range_chip.load_table(&mut layouter)?;

        Ok(())
    }
}
