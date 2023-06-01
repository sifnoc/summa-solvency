//! Parameters for using rate 2 Poseidon with the BN256 field.
//! Patterned after [halo2_gadgets::poseidon::primitives::fp]
//! The parameters can be reproduced by running the following Sage script from
//! [this repository](https://github.com/daira/pasta-hadeshash):
//!
//! ```text
//! $ sage generate_parameters_grain.sage 1 0 254 3 8 57 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001 --rust
//! ```
//!
//! where 1 means "prime field", 0 means "non-negative sbox", 254 is the bitsize
//! of the field, 3 is the Poseidon width (rate + 1), 8 is the number of full
//! rounds, 57 is the number of partial rounds, "0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001" is the prime modulus of bn256.
//! More info here => https://hackmd.io/@letargicus/SJOvx48Nn
use halo2_proofs::halo2curves::bn256::Fr as Fp;

// Number of round constants: 195
// Round constants for GF(p):
pub(crate) const ROUND_CONSTANTS: [[Fp; 3]; 65] = [
    [
        Fp::from_raw([
            0x8d21_d473_04cd_8e6e,
            0x14c4_993c_11bb_2993,
            0xd059_86d6_56f4_0c21,
            0x0ee9_a592_ba9a_9518,
        ]),
        Fp::from_raw([
            0x5696_fff4_0956_e864,
            0x887b_08d4_d008_68df,
            0x5986_5871_69fc_1bcd,
            0x00f1_4452_35f2_148c,
        ]),
        Fp::from_raw([
            0xe879_f389_0ecf_73f5,
            0x30c7_2873_0b7a_b36c,
            0x1f29_a058_d0fa_80b9,
            0x08df_f348_7e8a_c99e,
        ]),
    ],
    [
        Fp::from_raw([
            0x2096_6310_fadc_01d0,
            0x56c3_5342_c84b_da6e,
            0xc3ce_28f7_532b_13c8,
            0x2f27_be69_0fda_ee46,
        ]),
        Fp::from_raw([
            0x8b83_27be_bca1_6cf2,
            0xb763_fe04_b804_3ee4,
            0x2416_bebf_3d4f_6234,
            0x2b2a_e1ac_f68b_7b8d,
        ]),
        Fp::from_raw([
            0xe64b_44c7_dbf1_1cfa,
            0x5952_c175_ab6b_03ea,
            0xcca5_eac0_6f97_d4d5,
            0x0319_d062_072b_ef7e,
        ]),
    ],
    [
        Fp::from_raw([
            0x8ef7_b387_bf28_526d,
            0xc8b7_bf27_ad49_c629,
            0x8a37_6df8_7af4_a63b,
            0x2881_3dca_ebae_aa82,
        ]),
        Fp::from_raw([
            0x1509_28ad_ddf9_cb78,
            0x2033_8652_00c3_52bc,
            0xf181_bf38_e1c1_d40d,
            0x2727_673b_2ccb_c903,
        ]),
        Fp::from_raw([
            0xb8fb_9e31_e65c_c632,
            0x6efb_d43e_3405_87d6,
            0xe74a_bd2b_2a14_94cd,
            0x234e_c45c_a277_27c2,
        ]),
    ],
    [
        Fp::from_raw([
            0xcd99_ff6e_8797_d428,
            0xab10_a815_0a33_7b1c,
            0x7f86_2cb2_cf7c_f760,
            0x15b5_2534_031a_e18f,
        ]),
        Fp::from_raw([
            0xd701_d4ee_cf68_d1f6,
            0x8e0e_8a8d_1b58_b132,
            0x5ed9_a3d1_86b7_9ce3,
            0x0dc8_fad6_d9e4_b35f,
        ]),
        Fp::from_raw([
            0x9780_5518_a47e_4d9c,
            0xea4e_b378_f62e_1fec,
            0x600f_705f_ad3f_b567,
            0x1bcd_95ff_c211_fbca,
        ]),
    ],
    [
        Fp::from_raw([
            0x17cb_978d_069d_e559,
            0xc76d_a36c_2578_9378,
            0xe9ef_f81b_016f_c34d,
            0x1052_0b0a_b721_cadf,
        ]),
        Fp::from_raw([
            0xe88a_9eb8_1f56_27f6,
            0x2932_4980_75fe_d0ac,
            0x9b25_7d8e_d5fb_baf4,
            0x1f6d_4814_9b8e_7f7d,
        ]),
        Fp::from_raw([
            0xca34_bdb5_460c_8705,
            0xfff8_dc1c_816f_0dc9,
            0xd29e_00ef_35a2_089b,
            0x1d96_55f6_5230_9014,
        ]),
    ],
    [
        Fp::from_raw([
            0x8fe3_d418_5697_cc7d,
            0xa731_ff67_e470_3205,
            0xb051_f7b1_cd43_a99b,
            0x04df_5a56_ff95_bcaf,
        ]),
        Fp::from_raw([
            0xf6ec_282b_6e4b_e828,
            0x8690_a10a_8c84_24a7,
            0x151b_3d29_0ced_af14,
            0x0672_d995_f8ff_f640,
        ]),
        Fp::from_raw([
            0x9fc1_d820_9b5c_75b9,
            0x0c9a_9dcc_06f2_708e,
            0xb212_00d7_ffaf_dd5f,
            0x0999_52b4_1488_4454,
        ]),
    ],
    [
        Fp::from_raw([
            0x83fd_0e84_3a6b_9fa6,
            0x48e4_3586_a9b4_cd91,
            0x7c48_3143_ba8d_4694,
            0x052c_ba22_55df_d00c,
        ]),
        Fp::from_raw([
            0x1607_7cb9_3c46_4ddc,
            0x82de_5570_7251_ad77,
            0xb0bd_7471_2b79_99af,
            0x0b8b_adee_690a_db8e,
        ]),
        Fp::from_raw([
            0xb963_d0a8_e4b2_bdd1,
            0x49c1_5d60_683a_8050,
            0x5a1e_e651_020c_07c7,
            0x119b_1590_f133_07af,
        ]),
    ],
    [
        Fp::from_raw([
            0xce15_be0b_fb4a_8d09,
            0x2c4a_cfc8_84ef_4ee5,
            0x2529_d36b_e0f6_7b83,
            0x0315_0b7c_d6d5_d17b,
        ]),
        Fp::from_raw([
            0xbe69_cb31_7c9e_a565,
            0x5374_efb8_3d80_898a,
            0x3cf1_951f_1739_1235,
            0x2cc6_182c_5e14_546e,
        ]),
        Fp::from_raw([
            0x92d2_cd73_111b_f0f9,
            0x4218_cade_dac1_4e2b,
            0x50cf_e129_a404_b376,
            0x0050_3255_1e63_78c4,
        ]),
    ],
    [
        Fp::from_raw([
            0x88f9_da2c_c282_76b5,
            0x6469_c399_fcc0_69fb,
            0xbb14_7e97_2ebc_b951,
            0x2332_37e3_289b_aa34,
        ]),
        Fp::from_raw([
            0xe80c_2d4c_24d6_0280,
            0x2303_7f21_b34a_e5a4,
            0xc980_d316_74bf_be63,
            0x05c8_f4f4_ebd4_a6e3,
        ]),
        Fp::from_raw([
            0xee1f_09b2_590f_c65b,
            0x52bc_f35e_f3ae_ed91,
            0xba05_d818_a319_f252,
            0x0a7b_1db1_3042_d396,
        ]),
    ],
    [
        Fp::from_raw([
            0x5df5_4236_5a40_4ec0,
            0xf156_e2b0_86ff_47dc,
            0xb142_9657_2c9d_32db,
            0x2a73_b71f_9b21_0cf5,
        ]),
        Fp::from_raw([
            0x76a7_60bb_5c50_c460,
            0xec18_f2c4_dbe7_f229,
            0x9351_07e9_ffc9_1dc3,
            0x1ac9_b041_7abc_c9a1,
        ]),
        Fp::from_raw([
            0x9015_ee04_6dc9_3fc0,
            0x269f_3e4d_6cb1_0434,
            0x3fab_b076_707e_f479,
            0x12c0_339a_e083_7482,
        ]),
    ],
    [
        Fp::from_raw([
            0x8246_682e_56e9_a28e,
            0x5290_0aa3_253b_aac6,
            0x7f5b_18db_4e1e_704f,
            0x0b74_75b1_02a1_65ad,
        ]),
        Fp::from_raw([
            0x32ab_3aa8_8d7f_8448,
            0x7c84_3e37_9366_f2ea,
            0xdb1c_5e49_f6e8_b891,
            0x037c_2849_e191_ca3e,
        ]),
        Fp::from_raw([
            0x45fd_b176_a716_346f,
            0xd520_6c5c_93a0_7dc1,
            0xe926_7466_1e21_7e9b,
            0x05a6_811f_8556_f014,
        ]),
    ],
    [
        Fp::from_raw([
            0x7b67_5ef5_f38b_d66e,
            0x4076_e87a_7b28_83b4,
            0x6e94_7b75_d54e_9f04,
            0x29a7_95e7_d980_2894,
        ]),
        Fp::from_raw([
            0x507b_e199_981f_d22f,
            0x6e8c_7382_c8a1_585c,
            0x45a3_857a_fc18_f582,
            0x2043_9a0c_84b3_22eb,
        ]),
        Fp::from_raw([
            0x4a2a_6f2a_0982_c887,
            0xbb50_f277_99a8_4b6d,
            0x94ec_2050_c737_1ff1,
            0x2e0b_a8d9_4d9e_cf4a,
        ]),
    ],
    [
        Fp::from_raw([
            0xe6d0_ddcc_a17d_71c8,
            0x1782_2cd2_1090_48d2,
            0xca38_eb7c_ce82_2b45,
            0x143f_d115_ce08_fb27,
        ]),
        Fp::from_raw([
            0xc843_2362_3be9_caf1,
            0xf861_1659_323d_bcbf,
            0x5796_8dbb_dcf8_13cd,
            0x0c64_cbec_b1c7_34b8,
        ]),
        Fp::from_raw([
            0xf142_6cef_9403_da53,
            0xe74f_348d_62c2_b670,
            0x46fc_a925_c163_ff5a,
            0x028a_3058_47c6_83f6,
        ]),
    ],
    [
        Fp::from_raw([
            0x24d6_755b_5db9_e30c,
            0x6a6b_cb64_d894_27b8,
            0x5fa9_40ab_4c43_80f2,
            0x2e4e_f510_ff0b_6fda,
        ]),
        Fp::from_raw([
            0xb963_84f5_0579_400e,
            0x8925_b4f6_d033_b078,
            0x63d7_9270_c956_ce3b,
            0x0081_c95b_c433_84e6,
        ]),
        Fp::from_raw([
            0xba8a_9f40_23a0_bb38,
            0xe249_1b34_9c03_9a0b,
            0x187e_2fad_e687_e05e,
            0x2ed5_f0c9_1cbd_9749,
        ]),
    ],
    [
        Fp::from_raw([
            0x990f_01f3_3a73_5206,
            0x3448_a22c_7623_4c8c,
            0x4bbf_374e_d5aa_e2f0,
            0x3050_9991_f88d_a350,
        ]),
        Fp::from_raw([
            0xa752_9094_424e_c6ad,
            0xf0a1_119f_b206_7b41,
            0x221b_7c4d_49a3_56b9,
            0x1c3f_20fd_5540_9a53,
        ]),
        Fp::from_raw([
            0x1708_87b4_7ddc_b96c,
            0xc46b_b221_3e8e_131e,
            0x0495_1445_9b6e_18ee,
            0x10b4_e7f3_ab5d_f003,
        ]),
    ],
    [
        Fp::from_raw([
            0x039a_a350_2e43_adef,
            0xdd80_f804_c077_d775,
            0x3ddd_543d_891c_2abd,
            0x2a19_8297_9c3f_f7f4,
        ]),
        Fp::from_raw([
            0x5cad_0f13_15bd_5c91,
            0xba43_1ebc_396c_9af9,
            0xfedd_bead_56d6_d55d,
            0x1c74_ee64_f15e_1db6,
        ]),
        Fp::from_raw([
            0x9c2f_e45a_0ae1_46a0,
            0x9e4f_2e8b_8270_8cfa,
            0xeab9_303c_ace0_1b4b,
            0x0753_3ec8_50ba_7f98,
        ]),
    ],
    [
        Fp::from_raw([
            0x8a11_abf3_764c_0750,
            0x285c_68f4_2d42_c180,
            0xa151_e4ee_af17_b154,
            0x2157_6b43_8e50_0449,
        ]),
        Fp::from_raw([
            0x743d_6930_836d_4a9e,
            0xbce8_384c_815f_0906,
            0x08ad_5ca1_93d6_2f10,
            0x2f17_c055_9b8f_e796,
        ]),
        Fp::from_raw([
            0xe665_b0b1_b7e2_730e,
            0x9775_a420_1318_474a,
            0xa79e_8aae_9461_70bc,
            0x2d47_7e38_62d0_7708,
        ]),
    ],
    [
        Fp::from_raw([
            0xd89b_e0f5_b274_7eab,
            0xafba_2266_c38f_5abc,
            0x90e0_9557_7984_f291,
            0x162f_5243_9670_64c3,
        ]),
        Fp::from_raw([
            0x7777_a700_9239_3311,
            0xd7a8_596a_87f2_9f8a,
            0x264e_cd2c_8ae5_0d1a,
            0x2b4c_b233_ede9_ba48,
        ]),
        Fp::from_raw([
            0x4254_e7c3_5e03_b07a,
            0x6db2_eece_6d85_c4cf,
            0x1dba_f8f4_6228_5477,
            0x2c8f_bcb2_dd85_73dc,
        ]),
    ],
    [
        Fp::from_raw([
            0xe5e8_8db8_7094_9da9,
            0x9e1b_61e9_f601_e9ad,
            0xf2ff_453f_0cd5_6b19,
            0x1d6f_3477_25e4_816a,
        ]),
        Fp::from_raw([
            0x4cd4_9af5_c456_5529,
            0xf9e6_ac02_b68d_3132,
            0xebc2_d8b3_df5b_913d,
            0x204b_0c39_7f4e_be71,
        ]),
        Fp::from_raw([
            0x4ff8_fb75_bc79_c502,
            0x9ecb_827c_d7dc_2553,
            0x4f11_49b3_c63c_3c2f,
            0x0c4c_b9dc_3c4f_d817,
        ]),
    ],
    [
        Fp::from_raw([
            0x9a61_6ddc_45bc_7b54,
            0x1e5c_4947_5279_e063,
            0xa254_1647_4f49_3030,
            0x174a_d61a_1448_c899,
        ]),
        Fp::from_raw([
            0x3a98_16d4_9a38_d2ef,
            0xeaaa_28c1_77cc_0fa1,
            0xf759_df4e_c2f3_cde2,
            0x1a96_177b_cf4d_8d89,
        ]),
        Fp::from_raw([
            0x8242_ace3_60b8_a30a,
            0x0520_2c12_6a23_3c1a,
            0xd0ef_8054_bc60_c4ff,
            0x066d_04b2_4331_d71c,
        ]),
    ],
    [
        Fp::from_raw([
            0x2703_7a62_aa1b_d804,
            0x381c_c65f_72e0_2ad5,
            0x2195_7828_71c6_dd3b,
            0x2a4c_4fc6_ec0b_0cf5,
        ]),
        Fp::from_raw([
            0xe55a_fc01_219f_d649,
            0x5e72_7f84_46f6_d9d7,
            0x47e9_f2e1_4a7c_edc9,
            0x13ab_2d13_6ccf_37d4,
        ]),
        Fp::from_raw([
            0x4c2e_3e86_9acc_6a9a,
            0xc1b0_4fce_c26f_5519,
            0x19d2_4d84_3dc8_2769,
            0x1121_552f_ca26_0616,
        ]),
    ],
    [
        Fp::from_raw([
            0x09a5_546c_7c97_cff1,
            0xa6cd_267d_595c_4a89,
            0x889b_c817_15c3_7d77,
            0x00ef_6533_22b1_3d6c,
        ]),
        Fp::from_raw([
            0x845a_ca35_d8a3_97d3,
            0x400c_776d_6525_95d9,
            0x8b26_1d8b_a740_51e6,
            0x0e25_483e_45a6_6520,
        ]),
        Fp::from_raw([
            0x4644_8db9_79ee_ba89,
            0x395a_c3d4_dde9_2d8c,
            0x2452_6465_9e15_d88e,
            0x29f5_36dc_b9dd_7682,
        ]),
    ],
    [
        Fp::from_raw([
            0x0e45_6baa_ce0f_a5be,
            0x5a12_4e27_80bb_ea17,
            0xdfda_3357_5dbd_bd88,
            0x2a56_ef9f_2c53_feba,
        ]),
        Fp::from_raw([
            0xee41_6240_a8cb_9af1,
            0xf2ae_2999_a467_62e8,
            0xecfb_7a2d_17b5_c409,
            0x1c83_61c7_8eb5_cf5d,
        ]),
        Fp::from_raw([
            0xd3d0_ab4b_e743_19c5,
            0x83e8_e68a_7645_07bf,
            0xc047_3089_aaf0_206b,
            0x151a_ff5f_38b2_0a0f,
        ]),
    ],
    [
        Fp::from_raw([
            0xe76e_4761_5b51_f100,
            0xa9f5_2fc8_c8b6_cdd1,
            0xc1b2_39c8_8f7f_9d43,
            0x04c6_187e_41ed_881d,
        ]),
        Fp::from_raw([
            0x9e80_1b7d_dc9c_2967,
            0x4b81_c61e_d157_7644,
            0x10d8_4331_f6fb_6d53,
            0x13b3_7bd8_0f4d_27fb,
        ]),
        Fp::from_raw([
            0x9321_ceb1_c4e8_a8e4,
            0x2ce3_664c_2a52_032c,
            0xf578_bfbd_32c1_7b7a,
            0x01a5_c536_273c_2d9d,
        ]),
    ],
    [
        Fp::from_raw([
            0x8322_3906_5b7c_3b02,
            0x4a9a_2c66_6b97_26da,
            0x5ad0_5f5d_7acb_950b,
            0x2ab3_5618_34ca_7383,
        ]),
        Fp::from_raw([
            0x9f7e_d516_a597_b646,
            0xacaf_6af4_e95d_3bf6,
            0x200f_e6d6_86c0_d613,
            0x1d4d_8ec2_91e7_20db,
        ]),
        Fp::from_raw([
            0x1514_c9c8_0b65_af1d,
            0xb925_3512_40a0_4b71,
            0x8f57_84fe_7919_fd2b,
            0x0412_94d2_cc48_4d22,
        ]),
    ],
    [
        Fp::from_raw([
            0x0429_71dd_90e8_1fc6,
            0x98f5_7939_d126_e392,
            0x1c4f_a715_991f_0048,
            0x154a_c98e_0170_8c61,
        ]),
        Fp::from_raw([
            0x4524_563b_c6ea_4da4,
            0x50b3_684c_88f8_b0b0,
            0x3eed_d840_93ae_f510,
            0x0b33_9d8a_cca7_d4f8,
        ]),
        Fp::from_raw([
            0x81ed_95b5_0839_c82e,
            0x98f0_e71e_aff4_a7dd,
            0x54a4_f84c_fbab_3445,
            0x0955_e49e_6610_c942,
        ]),
    ],
    [
        Fp::from_raw([
            0x3525_401e_a065_4626,
            0xa9a6_f41e_6f53_5c6f,
            0x26b9_e222_06f1_5abc,
            0x0674_6a61_56eb_a544,
        ]),
        Fp::from_raw([
            0xac91_7c7f_f320_77fb,
            0x38e5_790e_2bd0_a196,
            0x496f_3820_c549_c278,
            0x0f18_f5a0_ecd1_423c,
        ]),
        Fp::from_raw([
            0x2a73_8223_d6f7_6e13,
            0x4bb5_6358_3ede_7bc9,
            0x8ac5_9eff_5beb_261e,
            0x04f6_eeca_1751_f730,
        ]),
    ],
    [
        Fp::from_raw([
            0xc176_8d26_fc0b_3758,
            0x8811_eb11_6fb3_e45b,
            0xc1a3_ec4d_a3cd_ce03,
            0x2b56_9733_64c4_c4f5,
        ]),
        Fp::from_raw([
            0x83fe_b65d_437f_29ef,
            0x8e13_92b3_8571_6a5d,
            0xdcd7_6b89_804b_1bcb,
            0x1237_69dd_49d5_b054,
        ]),
        Fp::from_raw([
            0x9425_7b2f_b01c_63e9,
            0xa989_f644_6471_1509,
            0x88ee_52b9_1169_aace,
            0x2147_b424_fc48_c80a,
        ]),
    ],
    [
        Fp::from_raw([
            0xea54_ad89_7ceb_e54d,
            0x647e_6f34_ad42_43c2,
            0x1a6c_5505_ea33_2a29,
            0x0fdc_1f58_548b_8570,
        ]),
        Fp::from_raw([
            0x944f_685c_c0a0_b1f2,
            0xbcef_f28c_5dbb_e0c3,
            0xdf68_abcf_0f77_86d4,
            0x1237_3a82_51fe_a004,
        ]),
        Fp::from_raw([
            0xdd8a_1f35_c1a9_0035,
            0xa642_756b_6af4_4203,
            0xad7e_a52f_f742_c9e8,
            0x21e4_f4ea_5f35_f85b,
        ]),
    ],
    [
        Fp::from_raw([
            0x8a81_934f_1bc3_b147,
            0xb573_6649_2f45_e90d,
            0xdfb4_7222_24d4_c462,
            0x1624_3916_d69d_2ca3,
        ]),
        Fp::from_raw([
            0xa13a_4159_cac0_4ac2,
            0xabc2_1566_e1a0_453c,
            0xf66f_9adb_c88b_4378,
            0x1efb_e46d_d7a5_78b4,
        ]),
        Fp::from_raw([
            0x3b67_2cc9_6a88_969a,
            0xd468_d552_5be6_6f85,
            0x8886_020e_23a7_f387,
            0x07ea_5e85_37cf_5dd0,
        ]),
    ],
    [
        Fp::from_raw([
            0xa9fe_16c0_b76c_00bc,
            0x650f_19a7_5e7c_e11c,
            0xb7b4_78a3_0f9a_5b63,
            0x05a8_c4f9_968b_8aa3,
        ]),
        Fp::from_raw([
            0x2d9d_57b7_2a32_e83f,
            0x3f78_18c7_01b9_c788,
            0xfbfe_59bd_345e_8dac,
            0x20f0_5771_2cc2_1654,
        ]),
        Fp::from_raw([
            0x9bd9_0b33_eb33_db69,
            0x6dcd_8e88_d01d_4901,
            0x9672_f8c6_7fee_3163,
            0x04a1_2ede_da9d_fd68,
        ]),
    ],
    [
        Fp::from_raw([
            0xe49e_c954_4ccd_101a,
            0xbd13_6ce5_091a_6767,
            0xe44f_1e54_25a5_1dec,
            0x27e8_8d8c_15f3_7dce,
        ]),
        Fp::from_raw([
            0x176c_41ee_433d_e4d1,
            0x6e09_6619_a770_3223,
            0xb8a5_c8c5_e95a_41f6,
            0x2fee_d17b_8428_5ed9,
        ]),
        Fp::from_raw([
            0x6972_b8bd_53af_f2b8,
            0x94e5_9429_1131_2a0d,
            0x4042_4142_0f72_9cf3,
            0x1ed7_cc76_edf4_5c7c,
        ]),
    ],
    [
        Fp::from_raw([
            0xdf28_74be_4546_6b1a,
            0xac67_8347_6144_cdca,
            0x157f_f8c5_86f5_660e,
            0x1574_2e99_b9bf_a323,
        ]),
        Fp::from_raw([
            0x284f_033f_27d0_c785,
            0x7710_7454_c6ec_0317,
            0xc895_fc68_87dd_f405,
            0x1aac_2853_87f6_5e82,
        ]),
        Fp::from_raw([
            0xec75_a965_54d6_7c77,
            0x832e_2e7a_4977_5f71,
            0xf9dd_adbd_b605_7357,
            0x2585_1c3c_845d_4790,
        ]),
    ],
    [
        Fp::from_raw([
            0x0ddc_cc3d_9f14_6a67,
            0x53b7_ebba_2c55_2337,
            0xce78_457d_b197_edf3,
            0x15a5_8215_65cc_2ec2,
        ]),
        Fp::from_raw([
            0x2f15_485f_28c7_1727,
            0xdcf6_4f36_0442_7750,
            0x0efa_7e31_a1db_5966,
            0x2411_d57a_4813_b998,
        ]),
        Fp::from_raw([
            0x5882_8b5e_f6cb_4c9b,
            0x47e9_a98e_12f4_cd25,
            0x13e3_35b8_c0b6_d2e6,
            0x002e_6f8d_6520_cd47,
        ]),
    ],
    [
        Fp::from_raw([
            0x3988_3460_9e03_15d2,
            0xaf8f_0e91_e2fe_1ed7,
            0x97da_00b6_16b0_fcd1,
            0x2ff7_bc8f_4380_cde9,
        ]),
        Fp::from_raw([
            0xe93b_e4fe_bb0d_3cbe,
            0x2e95_21f6_b7bb_68f1,
            0x5ee0_2724_471b_cd18,
            0x00b9_831b_9485_2559,
        ]),
        Fp::from_raw([
            0x7d77_adbf_0c9c_3512,
            0x1ca4_0864_8a47_43a8,
            0x8691_3b0e_57c0_4e01,
            0x0a2f_5376_8b8e_bf6a,
        ]),
    ],
    [
        Fp::from_raw([
            0x7f2a_2903_05e1_198d,
            0x0f59_9ff7_e94b_e69b,
            0x3a47_9f91_ff23_9e96,
            0x0024_8156_142f_d037,
        ]),
        Fp::from_raw([
            0x50eb_512a_2b2b_cda9,
            0x3971_96aa_6a54_2c23,
            0x28cf_8c02_ab3f_0c9a,
            0x171d_5620_b87b_fb13,
        ]),
        Fp::from_raw([
            0x9d10_45e4_ec34_a808,
            0x60c9_5217_2dd5_4dd9,
            0x7008_7c7c_10d6_fad7,
            0x170a_4f55_536f_7dc9,
        ]),
    ],
    [
        Fp::from_raw([
            0x482e_ca17_e2db_fae1,
            0xcc37_e38c_1cd2_11ba,
            0x2ef3_134a_ea04_336e,
            0x29ab_a33f_799f_e66c,
        ]),
        Fp::from_raw([
            0xb5ba_6503_69e6_4973,
            0xe70d_114a_03f6_a0e8,
            0xfdd1_bb19_4508_8d47,
            0x1e9b_c179_a4fd_d758,
        ]),
        Fp::from_raw([
            0x9c9e_1c43_bdaf_8f09,
            0xfeaa_d869_a9c4_b44f,
            0x58f7_f489_2dfb_0b5a,
            0x1dd2_6979_9b66_0fad,
        ]),
    ],
    [
        Fp::from_raw([
            0x5d1d_d2cb_0f24_af38,
            0x7ccd_426f_e869_c7c9,
            0x4011_81d0_2e15_459e,
            0x22cd_bc8b_7011_7ad1,
        ]),
        Fp::from_raw([
            0xd5ba_93b9_c7da_cefd,
            0xfd31_50f5_2ed9_4a7c,
            0x3a9f_57a5_5c50_3fce,
            0x0ef0_42e4_5477_1c53,
        ]),
        Fp::from_raw([
            0x3b30_4ffc_a62e_8284,
            0x1318_e8b0_8a03_59a0,
            0xf287_f303_6037_e885,
            0x1160_9e06_ad6c_8fe2,
        ]),
    ],
    [
        Fp::from_raw([
            0x08b0_8f5b_783a_a9af,
            0xfecd_58c0_76df_e427,
            0x9e75_3eea_427c_17b7,
            0x1166_d9e5_5461_6dba,
        ]),
        Fp::from_raw([
            0xf855_a888_357e_e466,
            0x177f_bf4c_d2ac_0b56,
            0x9341_3026_3544_13db,
            0x2de5_2989_431a_8595,
        ]),
        Fp::from_raw([
            0x74bf_01cf_5f71_e9ad,
            0xf51a_ee5b_17b8_e89d,
            0x9a6d_a492_f3a8_ac1d,
            0x3006_eb4f_fc7a_8581,
        ]),
    ],
    [
        Fp::from_raw([
            0x6234_4c82_2514_5086,
            0x2993_fe8f_0a46_39f9,
            0xfdcf_6fff_9e3f_6f42,
            0x2af4_1fbb_61ba_8a80,
        ]),
        Fp::from_raw([
            0x81b2_14ba_ce48_27c3,
            0x8718_ab27_889e_85e7,
            0xe5a6_b41a_8ebc_85db,
            0x119e_684d_e476_155f,
        ]),
        Fp::from_raw([
            0xcff7_84b9_7b3f_d800,
            0xb512_48c2_3828_f047,
            0x188b_ea59_ae36_3537,
            0x1835_b786_e2e8_925e,
        ]),
    ],
    [
        Fp::from_raw([
            0x6c40_e285_ab32_eeb6,
            0xd152_bac2_a790_5c92,
            0x4d79_4996_c643_3a20,
            0x2820_1a34_c594_dfa3,
        ]),
        Fp::from_raw([
            0x4a76_1f88_c22c_c4e7,
            0x864c_82eb_5711_8772,
            0x94e8_0fef_af78_b000,
            0x083e_fd7a_27d1_7510,
        ]),
        Fp::from_raw([
            0x9e07_9564_f61f_d13b,
            0x11c1_6df7_774d_d851,
            0x6158_e61c_eea2_7be8,
            0x0b6f_88a3_5771_9952,
        ]),
    ],
    [
        Fp::from_raw([
            0x1439_0e6e_e425_4f5b,
            0x5895_11ca_00d2_9e10,
            0x644f_66e1_d647_1a94,
            0x0ec8_68e6_d15e_51d9,
        ]),
        Fp::from_raw([
            0x00d9_37ab_84c9_8591,
            0xecd3_e74b_939c_d40d,
            0x1ac0_c9b3_ed2e_1142,
            0x2af3_3e3f_8667_7127,
        ]),
        Fp::from_raw([
            0x364c_e5e4_7951_f178,
            0x3456_8c54_7dd6_858b,
            0xd09b_5d96_1c6a_ce77,
            0x0b52_0211_f904_b5e7,
        ]),
    ],
    [
        Fp::from_raw([
            0xca22_8620_188a_1d40,
            0xa0c5_6ac4_270e_822c,
            0xd8db_58f1_0062_a92e,
            0x0b2d_722d_0919_a1aa,
        ]),
        Fp::from_raw([
            0xe006_1d1e_d6e5_62d4,
            0x57b5_4a99_91ca_38bb,
            0xd980_ceb3_7c24_53e9,
            0x1f79_0d4d_7f8c_f094,
        ]),
        Fp::from_raw([
            0xda92_ceb0_1e50_4233,
            0x0885_c162_35a2_a6a8,
            0xaea9_7cd3_85f7_8015,
            0x0171_eb95_dfbf_7d1e,
        ]),
    ],
    [
        Fp::from_raw([
            0x7623_0538_1b16_8873,
            0x790b_40de_fd2c_8650,
            0x329b_f688_5da6_6b9b,
            0x0c2d_0e3b_5fd5_7549,
        ]),
        Fp::from_raw([
            0x5d38_0305_4407_a18d,
            0x7cbc_afa5_89e2_83c3,
            0x4e5a_8228_b4e7_2b37,
            0x1162_fb28_689c_2715,
        ]),
        Fp::from_raw([
            0x1623_ef82_4971_1bc0,
            0x282c_5a92_a89e_1992,
            0x64ad_386a_91e8_310f,
            0x2f14_59b6_5dee_441b,
        ]),
    ],
    [
        Fp::from_raw([
            0xc243_f70d_1b53_cfbb,
            0xbc48_9d46_754e_b712,
            0x996d_7436_7d5c_d4c1,
            0x1e6f_f321_6b68_8c3d,
        ]),
        Fp::from_raw([
            0x7688_1f93_2647_8875,
            0xd741_a6f3_6cdc_2a05,
            0x6814_87d2_7d15_7802,
            0x01ca_8be7_3832_b8d0,
        ]),
        Fp::from_raw([
            0x0b9b_5de3_15f9_650e,
            0x6802_8608_0b10_cea0,
            0x86f9_76d5_bdf2_23dc,
            0x1f77_3570_6ffe_9fc5,
        ]),
    ],
    [
        Fp::from_raw([
            0x4745_ca83_8285_f019,
            0x21ac_10a3_d5f0_96ef,
            0x40a0_c2dc_e041_fba9,
            0x2522_b60f_4ea3_3076,
        ]),
        Fp::from_raw([
            0x8ce1_6c23_5572_575b,
            0x3418_cad4_f52b_6c3f,
            0x5255_075d_dc95_7f83,
            0x23f0_bee0_01b1_029d,
        ]),
        Fp::from_raw([
            0x66d9_4010_9308_2d59,
            0x5d14_2633_e9df_905f,
            0xcaac_2d44_555e_d568,
            0x2bc1_ae8b_8ddb_b81f,
        ]),
    ],
    [
        Fp::from_raw([
            0x8011_fcd6_ad72_205f,
            0x6237_1273_a07b_1fc9,
            0x7304_507b_8dba_3ed1,
            0x0f94_06b8_2965_64a3,
        ]),
        Fp::from_raw([
            0xcb12_6c8c_d995_f0a8,
            0x17e7_5b17_4a52_ee4a,
            0x67b7_2998_de90_714e,
            0x2360_a8eb_0cc7_defa,
        ]),
        Fp::from_raw([
            0x6dcb_bc27_67f8_8948,
            0xb481_5a5e_96df_8b00,
            0x804c_803c_baef_255e,
            0x1587_1a5c_ddea_d976,
        ]),
    ],
    [
        Fp::from_raw([
            0x4f95_7ccd_eefb_420f,
            0x362f_4f54_f723_7954,
            0x0a86_52dd_2f3b_1da0,
            0x193a_5676_6998_ee9e,
        ]),
        Fp::from_raw([
            0xe430_9805_e777_ae0f,
            0x3b2e_63c8_ad33_4834,
            0x2f9b_e56f_f4fa_b170,
            0x2a39_4a43_934f_8698,
        ]),
        Fp::from_raw([
            0xb416_6e88_76c0_d142,
            0x892c_d112_2344_3ba7,
            0x3e8b_635d_cb34_5192,
            0x1859_954c_feb8_695f,
        ]),
    ],
    [
        Fp::from_raw([
            0x408d_3819_f4fe_d32b,
            0x2b11_bc25_d90b_bdca,
            0x0134_44db_cb99_f190,
            0x04e1_1817_6305_0e58,
        ]),
        Fp::from_raw([
            0x1f5e_5552_bfd0_5f23,
            0xb10e_b82d_b08b_5e8b,
            0x40c3_35ea_64de_8c5b,
            0x0fdb_253d_ee83_869d,
        ]),
        Fp::from_raw([
            0xa9d7_c5ba_e9b4_f1c0,
            0x75f0_8686_f1c0_8984,
            0xaa4e_fb62_3ade_ad62,
            0x058c_be8a_9a50_27bd,
        ]),
    ],
    [
        Fp::from_raw([
            0xd152_28b4_ccec_a59a,
            0x23b4_b83b_ef02_3ab0,
            0x497e_adb1_aeb1_f52b,
            0x1382_edce_9971_e186,
        ]),
        Fp::from_raw([
            0xe1e6_6346_01d9_e8b5,
            0x7f61_b8eb_99f1_4b77,
            0x0819_ca51_fd11_b0be,
            0x0346_4990_f045_c6ee,
        ]),
        Fp::from_raw([
            0xaa5b_c137_aeb7_0a58,
            0x6fca_b460_5db2_eb5a,
            0xfff3_3b41_f98f_f83c,
            0x23f7_bfc8_720d_c296,
        ]),
    ],
    [
        Fp::from_raw([
            0x1963_6158_bbaf_62f2,
            0x18c3_ffd5_e153_1a92,
            0x7e6e_94e7_f0e9_decf,
            0x0a59_a158_e3ee_c211,
        ]),
        Fp::from_raw([
            0xf4c2_3ed0_075f_d07b,
            0xe2c4_eba0_6542_0af8,
            0xb58b_f23b_312f_fd3c,
            0x06ec_54c8_0381_c052,
        ]),
        Fp::from_raw([
            0x962f_0ff9_ed1f_9d01,
            0xb093_40f7_a7bc_b1b4,
            0x476b_5664_8e86_7ec8,
            0x1188_72dc_832e_0eb5,
        ]),
    ],
    [
        Fp::from_raw([
            0x95e1_906b_5209_21b1,
            0x52e0_b0f0_e42d_7fea,
            0x5ad5_c7cb_a7ad_59ed,
            0x13d6_9fa1_27d8_3416,
        ]),
        Fp::from_raw([
            0xfd8a_49f1_9f10_c77b,
            0xde14_3942_fb71_dc55,
            0x70b1_c687_7a73_d21b,
            0x169a_177f_63ea_6812,
        ]),
        Fp::from_raw([
            0xfb7e_9a5a_7450_544d,
            0x3abe_b032_b922_f66f,
            0xef42_f287_adce_40d9,
            0x04ef_5159_1c6e_ad97,
        ]),
    ],
    [
        Fp::from_raw([
            0xd5f4_5ee6_dd0f_69ec,
            0x19ec_6180_5d4f_03ce,
            0x0ecd_7ca7_03fb_2e3b,
            0x256e_175a_1dc0_7939,
        ]),
        Fp::from_raw([
            0xa002_813d_3e2c_eeb2,
            0x75cc_360d_3205_dd2d,
            0xe5f2_af41_2ff6_004f,
            0x3010_2d28_636a_bd5f,
        ]),
        Fp::from_raw([
            0x1fd3_1be1_82fc_c792,
            0x0443_a3fa_99be_f4a3,
            0x1c07_14bc_73eb_1bf4,
            0x1099_8e42_dfcd_3bbf,
        ]),
    ],
    [
        Fp::from_raw([
            0xecad_76f8_79e3_6860,
            0x9f33_62ea_f4d5_82ef,
            0x25fa_7d24_b598_a1d8,
            0x193e_dd8e_9fcf_3d76,
        ]),
        Fp::from_raw([
            0xf266_4d7a_a51f_0b5d,
            0xd1c7_a561_ce61_1425,
            0xd036_8ce8_0b7b_3347,
            0x1816_8afd_34f2_d915,
        ]),
        Fp::from_raw([
            0x29e2_e95b_33ea_6111,
            0xa328_ec77_bc33_626e,
            0x0c01_7656_ebe6_58b6,
            0x2938_3c01_ebd3_b6ab,
        ]),
    ],
    [
        Fp::from_raw([
            0x00bf_573f_9010_c711,
            0x702d_b6e8_6fb7_6ab6,
            0xa1f4_ae5e_7771_a64a,
            0x1064_6d2f_2603_de39,
        ]),
        Fp::from_raw([
            0x64d0_242d_cb11_17fb,
            0x2f90_c25b_40da_7b38,
            0xf575_f139_5a55_bf13,
            0x0beb_5e07_d1b2_7145,
        ]),
        Fp::from_raw([
            0xdffb_f018_d96f_a336,
            0x30f9_5bb2_e54b_59ab,
            0xdc0d_3eca_d62b_5c88,
            0x16d6_8525_2078_c133,
        ]),
    ],
    [
        Fp::from_raw([
            0xfd67_2dd6_2047_f01a,
            0x0a55_5bbb_ec21_ddfa,
            0x3c74_154e_0404_b4b4,
            0x0a6a_bd1d_8339_38f3,
        ]),
        Fp::from_raw([
            0x70a6_f19b_34cf_1860,
            0xb12d_ffee_c450_3172,
            0x8ea1_2a4c_2ded_c8fe,
            0x1a67_9f5d_36eb_7b5c,
        ]),
        Fp::from_raw([
            0xfbc7_592e_3f1b_93d6,
            0x26a4_23ea_da4e_8f6f,
            0x3974_d50e_0ebf_de47,
            0x0980_fb23_3bd4_56c2,
        ]),
    ],
    [
        Fp::from_raw([
            0x03eb_acb5_c312_c72b,
            0xcece_3d56_28c9_2820,
            0xbf18_10af_93a3_8fc0,
            0x161b_4223_2e61_b84c,
        ]),
        Fp::from_raw([
            0xd092_03db_47de_1a0b,
            0x493f_0978_7f15_64e5,
            0x950f_7d47_a60d_5e6a,
            0x0ada_10a9_0c7f_0520,
        ]),
        Fp::from_raw([
            0xb50d_db9a_f407_f451,
            0xd3f0_7a8a_2b4e_121b,
            0x3203_45a2_9ac4_238e,
            0x1a73_0d37_2310_ba82,
        ]),
    ],
    [
        Fp::from_raw([
            0xfbda_10ef_58e8_c556,
            0x9083_77fe_aba5_c4df,
            0x8170_64c3_69dd_a7ea,
            0x2c81_20f2_68ef_054f,
        ]),
        Fp::from_raw([
            0x6e7b_8649_a496_8f70,
            0xb930_e953_13bc_b73e,
            0xa57c_0078_9c68_4217,
            0x1c7c_8824_f758_753f,
        ]),
        Fp::from_raw([
            0xb47b_27fa_3fd1_cf77,
            0xf400_ad8b_491e_b3f7,
            0x8e39_e407_7a74_faa0,
            0x2cd9_ed31_f5f8_691c,
        ]),
    ],
    [
        Fp::from_raw([
            0x854a_e239_18a2_2eea,
            0xa5e0_22ac_321c_a550,
            0xcf60_d92f_5761_8399,
            0x23ff_4f9d_4681_3457,
        ]),
        Fp::from_raw([
            0xdff1_ea58_f180_426d,
            0xaf5a_2c51_0352_9407,
            0xceec_e640_5ddd_d9d0,
            0x0994_5a5d_147a_4f66,
        ]),
        Fp::from_raw([
            0x8a6d_d223_ec6f_c630,
            0x7c7d_a6ea_a29d_3f26,
            0xb676_60c6_b771_b90f,
            0x188d_9c52_8025_d4c2,
        ]),
    ],
    [
        Fp::from_raw([
            0xe0c0_d8dd_f4f0_f47f,
            0xdba7_d926_d363_3595,
            0x81f6_8311_431d_8734,
            0x3050_e379_9659_6b7f,
        ]),
        Fp::from_raw([
            0x9d82_9518_d30a_fd78,
            0x6cea_e546_1e3f_95d8,
            0x1600_ca81_02c3_5c42,
            0x15af_1169_3968_30a9,
        ]),
        Fp::from_raw([
            0x0428_4da3_320d_8acc,
            0xdae9_33e3_5146_6b29,
            0xa06d_9f37_f873_d985,
            0x1da6_d098_8543_2ea9,
        ]),
    ],
    [
        Fp::from_raw([
            0xe546_ee41_1dda_a9cb,
            0x4e4f_ad3d_be65_8945,
            0xf5f8_acf3_3921_124e,
            0x2796_ea90_d269_af29,
        ]),
        Fp::from_raw([
            0x7cb0_319e_01d3_2d60,
            0x1e15_612e_c8e9_304a,
            0x0325_c8b3_3077_42f0,
            0x202d_7dd1_da0f_6b4b,
        ]),
        Fp::from_raw([
            0xa29d_ace4_c0f8_be5f,
            0xa2d7_f9c7_88f4_c831,
            0x156a_952b_a263_d672,
            0x096d_6790_d05b_b759,
        ]),
    ],
    [
        Fp::from_raw([
            0x6379_8cb1_447d_25a4,
            0x438d_a23c_e5b1_3e19,
            0x8380_8965_275d_877b,
            0x054e_fa1f_65b0_fce2,
        ]),
        Fp::from_raw([
            0x64cc_f6e1_8e41_65f1,
            0xd8aa_6901_13b2_e148,
            0xdb33_08c2_9802_deb9,
            0x1b16_2f83_d917_e93e,
        ]),
        Fp::from_raw([
            0xc5ce_b745_a050_6edc,
            0xedfe_fc14_66cc_568e,
            0xfd9f_1cdd_2a0d_e39e,
            0x21e5_241e_1256_4dd6,
        ]),
    ],
    [
        Fp::from_raw([
            0x7b43_49e1_0e4b_df08,
            0xcb73_ab5f_87e1_6192,
            0x226a_80ee_17b3_6abe,
            0x1cfb_5662_e8cf_5ac9,
        ]),
        Fp::from_raw([
            0x29c5_3f66_6eb2_4100,
            0x2c99_af34_6220_ac01,
            0xbae6_d8d1_ecb3_73b6,
            0x0f21_177e_302a_771b,
        ]),
        Fp::from_raw([
            0xbcef_7e1f_515c_2320,
            0xc423_6aed_e629_0546,
            0xaffb_0dd7_f71b_12be,
            0x1671_5223_7460_6992,
        ]),
    ],
    [
        Fp::from_raw([
            0xd419_d2a6_92ca_d870,
            0xbe2e_c9e4_2c5c_c8cc,
            0x2eb4_cf24_501b_fad9,
            0x0fa3_ec5b_9488_259c,
        ]),
        Fp::from_raw([
            0x85e8_c57b_1ab5_4bba,
            0xd36e_dce8_5c64_8cc0,
            0x57cb_266c_1506_080e,
            0x193c_0e04_e0bd_2983,
        ]),
        Fp::from_raw([
            0xce14_ea2a_daba_68f8,
            0x9f6f_7291_cd40_6578,
            0x7e91_2830_6dcb_c3c9,
            0x102a_df8e_f747_35a2,
        ]),
    ],
    [
        Fp::from_raw([
            0x40a6_d0cb_70c3_eab1,
            0x316a_a24b_fbdd_23ae,
            0xe2a5_4d6f_1ad9_45b1,
            0x0fe0_af78_58e4_9859,
        ]),
        Fp::from_raw([
            0xe8a5_ea73_4479_8d22,
            0x2da5_f1da_a9eb_defd,
            0x0853_6a22_2084_3f4e,
            0x216f_6717_bbc7_dedb,
        ]),
        Fp::from_raw([
            0xf88e_2e42_2832_5161,
            0x3c23_b2ac_773c_6b3e,
            0x4a3e_6943_9191_8a1b,
            0x1da5_5cc9_00f0_d21f,
        ]),
    ],
];
// n: 254
// t: 3
// N: 762
// Result Algorithm 1:
//  [True, 0]
// Result Algorithm 2:
//  [True, None]
// Result Algorithm 3:
//  [True, None]
// Prime number: 0x0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001
// MDS matrix:
pub(crate) const MDS: [[Fp; 3]; 3] = [
    [
        Fp::from_raw([
            0xfedb_6859_2ba8_118b,
            0x94be_7c11_ad24_378b,
            0xb2b7_0caf_5c36_a7b1,
            0x109b_7f41_1ba0_e4c9,
        ]),
        Fp::from_raw([
            0xd6c6_4543_dc49_03e0,
            0x9314_dc9f_dbde_ea55,
            0x6ae1_1942_4fdd_bcbc,
            0x16ed_41e1_3bb9_c0c6,
        ]),
        Fp::from_raw([
            0x791a_93b7_4e36_736d,
            0xf706_ab64_0ceb_247b,
            0xf617_e7dc_bfe8_2e0d,
            0x2b90_bba0_0fca_0589,
        ]),
    ],
    [
        Fp::from_raw([
            0xd629_40bc_de0b_d771,
            0x2cc8_fdd1_415c_3dde,
            0xb9c3_6c76_4379_dbca,
            0x2969_f27e_ed31_a480,
        ]),
        Fp::from_raw([
            0x29b2_3116_87b1_fe23,
            0xb89d_743c_8c7b_9640,
            0x4c98_71c8_3296_3dc1,
            0x2e24_19f9_ec02_ec39,
        ]),
        Fp::from_raw([
            0xc8aa_cc55_a0f8_9bfa,
            0x148d_4e10_9f5f_b065,
            0x9731_5876_690f_053d,
            0x1010_71f0_0323_79b6,
        ]),
    ],
    [
        Fp::from_raw([
            0x3262_44ee_65a1_b1a7,
            0xe6cd_79e2_8c5b_3753,
            0x0d5f_9e65_4638_065c,
            0x1430_21ec_686a_3f33,
        ]),
        Fp::from_raw([
            0xb16c_dfab_c8ee_2911,
            0xd057_e12e_58e7_d7b6,
            0x82a7_0eff_08a6_fd99,
            0x176c_c029_695a_d025,
        ]),
        Fp::from_raw([
            0x7327_9cd7_1d25_d5e0,
            0xa644_4703_0704_3f77,
            0x17ba_7fee_3802_593f,
            0x19a3_fc0a_5670_2bf4,
        ]),
    ],
];
// Inverse MDS matrix:
pub(crate) const MDS_INV: [[Fp; 3]; 3] = [
    [
        Fp::from_raw([
            0x7dfe_a07d_03f8_2f26,
            0x879a_5f36_b4ba_6dd1,
            0xb646_5d69_d3e1_2806,
            0x203d_1d35_1372_bf15,
        ]),
        Fp::from_raw([
            0x6e32_0628_bb03_2c22,
            0x48e4_2ce2_096b_3a1d,
            0x4b6a_d7fe_4620_063d,
            0x29b6_5372_1861_5bcb,
        ]),
        Fp::from_raw([
            0x1e2a_35a9_58b9_3a6a,
            0x54f5_1f71_1623_515b,
            0xb51b_d377_d7bb_55c0,
            0x1155_1257_de3d_4b5a,
        ]),
    ],
    [
        Fp::from_raw([
            0xdd13_1981_4f83_3b54,
            0xbf0d_c3bc_4fb6_2798,
            0x3d56_9912_c20f_1f82,
            0x29de_db1b_bf80_c886,
        ]),
        Fp::from_raw([
            0x24ca_8a66_304e_3849,
            0xbe03_67fe_c157_5cb1,
            0xd66c_7251_dc8f_56fb,
            0x130b_5914_3f4e_340c,
        ]),
        Fp::from_raw([
            0x1084_76d2_eeda_95f9,
            0x248c_8d6d_3b16_d4b4,
            0x6087_842f_b625_21a3,
            0x0c28_08c9_533e_2c52,
        ]),
    ],
    [
        Fp::from_raw([
            0xd288_9dc1_45c6_1609,
            0x29e1_387e_706c_f0de,
            0x9170_6fe0_9af2_2cfd,
            0x0173_249a_1c9e_ac25,
        ]),
        Fp::from_raw([
            0xd2e5_fb51_38e5_9350,
            0xd162_70b0_238f_3063,
            0xc82e_03ec_3cee_0cf1,
            0x0abc_7f15_8780_841e,
        ]),
        Fp::from_raw([
            0x5413_af26_963b_4700,
            0xe9f2_478c_2f28_ee94,
            0x8305_505a_af3b_497f,
            0x1738_a318_c863_1b6e,
        ]),
    ],
];
