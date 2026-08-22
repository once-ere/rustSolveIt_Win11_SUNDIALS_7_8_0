//! `sin`, `cos` — double-precision, host-independent.
//!
//! Translated from: glibc 2.39 `sysdeps/ieee754/dbl-64/s_sin.c`,
//! `sincostab.c`, `branred.c`, `usncs.h`, `branred.h` (IBM Accurate
//! Mathematical Library, "an ultimate sin and cos routine", ~0.55 ULP). The
//! multi-precision slow paths those routines used to carry were deleted
//! upstream in glibc 2.28; what is here is the whole of the 2.39 algorithm.
//!
//! # Which build of the C this reproduces
//!
//! On x86-64 glibc ifunc-dispatches `sin`/`cos` to `__sin_fma`/`__cos_fma`
//! (`sysdeps/x86_64/fpu/multiarch/s_sin-fma.c`), i.e. `s_sin.c` rebuilt with
//! `-mfma -mavx2 -ffp-contract=fast`. **`branred.c` is not rebuilt that way**
//! — there is no `branred-fma.c`, so `__branred` is the generic object
//! compiled against the SSE2 x86-64 baseline, which has no FMA at all. That
//! split is not cosmetic: contracting `t = x * split` in `__branred`'s
//! Veltkamp split would make `x1 = t - (t - x)` collapse to `x1 = x`
//! (because `x*split - x = x*2^27` is exact), `x2` would become zero and the
//! whole Payne–Hanek reduction would lose every guard bit. Measured against
//! the oracle: contracting `branred` costs ~391 000 of 4 000 000 sin inputs,
//! with errors of order 1, not 1 ulp.
//!
//! So: [`do_sin`], [`do_cos`], [`taylor_sin`], [`reduce_sincos`] and the two
//! entry points are contracted; [`branred`] is not, anywhere.
//!
//! # The contraction map inside the FMA build
//!
//! GCC's `widening_mul` pass contracts a multiply into an add/sub only when
//! *every* use of the product is an add/sub in the same basic block, and it
//! rewrites the uses in statement order — so a product whose only consumer
//! has already become an `.FMA` call is left as a bare multiply. Each site
//! below is annotated with the `.FMA`/`.FNMA`/`.FMS` that GCC emits. The
//! non-obvious ones:
//!
//! * `c = x*dx + xx*(cs2 + ...)` in [`do_sin`] fuses on the **first**
//!   multiply, `x*dx`; the polynomial's outer `xx * (...)` is then left
//!   unfused because its consumer is no longer a `PLUS_EXPR`.
//! * `s = x + (dx + x*xx*(...))` fuses `(x*xx)*poly` into the inner `dx +`,
//!   and the outer `x +` stays a plain add. In [`do_cos`] the same shape has
//!   no `dx`, so the single fma lands on the outer add instead.
//! * `t1 = xn*pp3` in [`reduce_sincos`] has **two** consumers, `y - t1` and
//!   `(y - t2) - t1`; both are subtractions, so GCC converts both and the
//!   multiply dies. Same for `t1 = xn*pp4`. This is the "all uses convert"
//!   rule, and it is why there is no bare `xn * pp3` below.
//! * `POLYNOMIAL(xx)*x - 0.5*dx` in [`taylor_sin`] becomes
//!   `.FMS (poly, x, 0.5*dx)`, i.e. the *first* multiply is the fused one
//!   and `0.5*dx` stays a rounded (exact) multiply.
//! * `t = x*hpinv + toint` in [`reduce_sincos`] *is* fused, unlike the
//!   superficially similar reduction step in [`crate::sundials_libm::exp`].
//!
//! SPDX-License-Identifier: LGPL-2.1-or-later

/* ---------------------------------------------------------------- usncs.h */

const S1: f64 = f64::from_bits(0xbfc5_5555_5555_5555); /* -0x1.5555555555555p-3  */
const S2: f64 = f64::from_bits(0x3f81_1111_1111_0ece); /*  0x1.1111111110ECEp-7  */
const S3: f64 = f64::from_bits(0xbf2a_01a0_19db_08b8); /* -0x1.A01A019DB08B8p-13 */
const S4: f64 = f64::from_bits(0x3ec7_1de2_7b9a_7ed9); /*  0x1.71DE27B9A7ED9p-19 */
const S5: f64 = f64::from_bits(0xbe5a_ddff_c2fc_df59); /* -0x1.ADDFFC2FCDF59p-26 */
const BIG: f64 = f64::from_bits(0x42c8_0000_0000_0000); /*  0x1.8p45   = 52776558133248 */
const HP0: f64 = f64::from_bits(0x3ff9_21fb_5444_2d18); /*  0x1.921FB54442D18p0   */
const HP1: f64 = f64::from_bits(0x3c91_a626_3314_5c07); /*  0x1.1A62633145C07p-54 */
const MP1: f64 = f64::from_bits(0x3ff9_21fb_5800_0000); /*  0x1.921FB58000000p0   */
const MP2: f64 = f64::from_bits(0xbe4d_de97_3c00_0000); /* -0x1.DDE973C000000p-27 */
const PP3: f64 = f64::from_bits(0xbc8c_b3b3_9800_0000); /* -0x1.CB3B398000000p-55 */
const PP4: f64 = f64::from_bits(0xbacd_747f_23e3_2ed7); /* -0x1.d747f23e32ed7p-83 */
const HPINV: f64 = f64::from_bits(0x3fe4_5f30_6dc9_c883); /* 0x1.45F306DC9C883p-1 */
const TOINT: f64 = f64::from_bits(0x4338_0000_0000_0000); /* 0x1.8p52 = 6755399441055744 */

/* ------------------------------- static const double sn3 … cs6 (s_sin.c) */

const SN3: f64 = f64::from_bits(0xbfc5_5555_5555_5515); /* -1.66666666666664880952546298448555E-01 */
const SN5: f64 = f64::from_bits(0x3f81_1110_e829_872f); /*  8.33333214285722277379541354343671E-03 */
const CS2: f64 = f64::from_bits(0x3fe0_0000_0000_0000); /*  4.99999999999999999999950396842453E-01 */
const CS4: f64 = f64::from_bits(0xbfa5_5555_5555_5535); /* -4.16666666666664434524222570944589E-02 */
const CS6: f64 = f64::from_bits(0x3f56_c16b_edd9_e239); /*  1.38888874007937613028114285595617E-03 */

/* ------------------------------------------------------------- branred.h */

const T576: f64 = f64::from_bits(0x63f0_0000_0000_0000); /* 2 ^ 576  */
const TM600: f64 = f64::from_bits(0x1a70_0000_0000_0000); /* 2 ^-600 */
const TM24: f64 = f64::from_bits(0x3e70_0000_0000_0000); /* 2 ^- 24  */
const BR_BIG: f64 = f64::from_bits(0x4338_0000_0000_0000); /*  6755399441055744  */
const BR_BIG1: f64 = f64::from_bits(0x4358_0000_0000_0000); /* 27021597764222976  */
const BR_HP0: f64 = f64::from_bits(0x3ff9_21fb_5444_2d18); /* 1.5707963267948966     */
const BR_HP1: f64 = f64::from_bits(0x3c91_a626_3314_5c07); /* 6.123233995736766e-17  */
const BR_MP1: f64 = f64::from_bits(0x3ff9_21fb_5800_0000); /* 1.5707963407039642     */
const BR_MP2: f64 = f64::from_bits(0xbe4d_de97_4000_0000); /* -1.3909067675399456e-08 */
/* dla.h: CN = 1 + 2**27, the Veltkamp splitting constant. */
const SPLIT: f64 = 134217729.0;

/* `__sincostab` of sincostab.c: 110 quadruples (sn, ssn, cs, ccs) for
k/128, k = 0..109. The C is a union of `int4 i[880]` / `double x[440]`;
this is the LITTLE_ENDI initialiser with each `lo, hi` pair packed into
one `lo | hi<<32`, machine-generated from the C — never retyped. */
const SINCOSTAB: [u64; 440] = [
    0x0000_0000_0000_0000, 0x0000_0000_0000_0000, 0x3ff0_0000_0000_0000, 0x0000_0000_0000_0000,
    0x3f7f_ffea_aaae_eeef, 0xbc1e_45e2_ec67_b77c, 0x3fef_ffc0_0015_5552, 0x3c8f_4a01_a019_6dae,
    0x3f8f_ffaa_aaee_eed5, 0xbc02_ab63_9a9f_0777, 0x3fef_ff00_0155_549f, 0x3c82_8a28_a03a_5ef3,
    0x3f97_ff70_0103_3255, 0x3bfe_fe2b_5152_7336, 0x3fef_fdc0_06bf_f7e6, 0x3c8a_e6da_e869_77bd,
    0x3f9f_feaa_aeee_e86f, 0xbc3c_d406_fb22_4ae2, 0x3fef_fc00_1555_27d3, 0xbc83_b544_92d8_9b5b,
    0x3fa3_feb2_b12d_45d5, 0x3c34_ec54_203d_1c11, 0x3fef_f9c0_3414_a7ba, 0x3c69_91f4_be6c_59bf,
    0x3fa7_fdc0_1032_fba9, 0xbc45_99bd_f46e_997a, 0x3fef_f700_6bfd_f99f, 0xbc78_b3b5_6064_8d5f,
    0x3fab_fc6d_7858_6dac, 0x3c18_e4fd_03db_f236, 0x3fef_f3c0_c810_3a31, 0x3c74_856d_bddc_0e66,
    0x3faf_faaa_eeed_4edb, 0xbc42_d16d_3268_4b69, 0x3fef_f001_5549_f4d3, 0x3c83_2838_7b99_426f,
    0x3fb1_fc34_3d80_8bef, 0xbc5f_3d32_e6f3_be4f, 0x3fef_ebc2_22a8_ef9f, 0x3c57_9349_34f5_4c77,
    0x3fb3_facb_12d1_755b, 0xbc59_2191_5299_468c, 0x3fef_e703_4129_ef6f, 0xbc6c_bf43_37c9_6f97,
    0x3fb5_f911_fd10_b737, 0xbc50_184f_02be_9102, 0x3fef_e1c4_c3c8_73eb, 0xbc35_a9c9_057c_4a02,
    0x3fb7_f701_0325_50e4, 0x3c3a_fc2d_1800_501a, 0x3fef_dc06_bf7e_6b9b, 0x3c83_1902_b535_f8db,
    0x3fb9_f490_2d55_d1f9, 0x3c52_696d_7eac_1dc1, 0x3fef_d5c9_4b43_e000, 0xbc62_e768_cb4f_92f9,
    0x3fbb_f1b7_8568_391d, 0x3c5e_9184_1dea_4cc8, 0x3fef_cf0c_800e_99b1, 0x3c6e_a3d7_86d1_86ac,
    0x3fbd_ee6f_16c1_cce6, 0xbc45_0f8e_2fb7_1673, 0x3fef_c7d0_78d1_bc88, 0x3c80_75d2_447d_b685,
    0x3fbf_eaae_ee86_ee36, 0xbc4a_fcb2_bcc6_f03b, 0x3fef_c015_527d_5bd3, 0x3c8b_68f3_5094_efb8,
    0x3fc0_f337_8ddd_71d1, 0x3c6d_8468_724f_0f9e, 0x3fef_b7db_2bfe_0695, 0x3c82_1dad_f4f6_5ab1,
    0x3fc1_f0d3_d7af_ceaf, 0xbc66_ef95_0997_69a5, 0x3fef_af22_263c_4bd3, 0xbc55_2ace_133a_2769,
    0x3fc2_ee28_5e4a_b88f, 0xbc6e_4d0f_05de_e058, 0x3fef_a5ea_641c_36f2, 0x3c40_4da6_ed17_cc7c,
    0x3fc3_eb31_2c5d_66cb, 0x3c64_7d66_6b66_cb91, 0x3fef_9c34_0a7c_c428, 0x3c8c_5b6b_063b_7462,
    0x3fc4_e7ea_4dc5_f27b, 0x3c59_49db_2ac0_72fc, 0x3fef_91ff_4037_4d01, 0xbc67_d03f_4d3a_9e4c,
    0x3fc5_e44f_cfa1_26f3, 0xbc66_f443_063f_89b6, 0x3fef_874c_2e1e_ecf6, 0xbc8c_6514_e133_2b16,
    0x3fc6_e05d_c05a_4d4c, 0xbbd3_2c5c_8b81_c940, 0x3fef_7c1a_feff_de24, 0xbc78_f55b_c475_40b1,
    0x3fc7_dc10_2fba_f2b5, 0x3c45_ab50_e23c_97c3, 0x3fef_706b_df9e_ce1c, 0xbc86_98c8_0c36_dcb4,
    0x3fc8_d763_2efa_a944, 0xbc62_0fa2_62cb_b953, 0x3fef_643e_feb8_2acd, 0x3c76_b00a_c1fe_28ac,
    0x3fc9_d252_d0ce_c312, 0x3c59_c43d_80b1_137d, 0x3fef_5794_8cff_6797, 0x3c6e_3a0d_3e03_b1d5,
    0x3fca_ccdb_297a_0765, 0xbc59_883b_57d6_cdeb, 0x3fef_4a6c_bd1e_3a79, 0x3c81_3df0_edae_bb57,
    0x3fcb_c6f8_4edc_6199, 0x3c69_c1a5_6a7b_0cab, 0x3fef_3cc7_c3b3_d16e, 0xbc62_1a3a_d28a_3494,
    0x3fcc_c0a6_5882_89a3, 0xbc68_68d0_9bc8_7c6b, 0x3fef_2ea5_d753_ffed, 0x3c8c_c421_5f56_d583,
    0x3fcd_b9e1_5fb5_a5d0, 0xbc63_2e20_d6cc_6fc2, 0x3fef_2007_3086_649f, 0x3c7b_9404_16c1_984b,
    0x3fce_b2a5_7f8a_e5a3, 0xbc60_be06_af57_2ceb, 0x3fef_10ec_09c5_873b, 0x3c8d_9072_762c_1283,
    0x3fcf_aaee_d4f3_1577, 0xbc61_5d88_508e_32b8, 0x3fef_0154_9f7d_eea1, 0x3c8d_3c1e_99e5_cafd,
    0x3fd0_515c_bf65_155c, 0xbc79_b8c2_9dfd_8ec8, 0x3fee_f141_300d_2f26, 0xbc82_aa1b_08de_d372,
    0x3fd0_cd00_cef3_6436, 0xbc79_fb0a_0c93_e2b5, 0x3fee_e0b1_fbc0_f11c, 0xbc4b_fd23_80bb_c3b1,
    0x3fd1_4861_aa94_ddeb, 0xbc6b_e881_b5b6_15a4, 0x3fee_cfa7_44d5_efa1, 0xbc55_6d0a_4af5_41d0,
    0x3fd1_c37d_64c6_b876, 0x3c74_6076_fe0d_cff5, 0x3fee_be21_4f76_efa8, 0xbc80_2f9f_12ba_543e,
    0x3fd2_3e52_111a_af36, 0xbc74_f080_334e_ff18, 0x3fee_ac20_61bb_af4f, 0x3c62_c1d5_3e94_658d,
    0x3fd2_b8dd_c43e_b49f, 0x3c61_5538_99f2_d807, 0x3fee_99a4_c3a7_cd83, 0xbc82_264b_1bc5_3ce8,
    0x3fd3_331e_9404_9f87, 0x3c7e_0cb6_b40c_302c, 0x3fee_86ae_bf29_a9ed, 0x3c89_397a_fdbb_58a7,
    0x3fd3_ad12_9769_d3d8, 0x3c00_3d55_0487_8398, 0x3fee_733e_a019_3d40, 0xbc86_428b_3546_ce13,
    0x3fd4_26b7_e69e_e697, 0xbc7f_09c7_5705_c59f, 0x3fee_5f54_b436_e9d0, 0x3c87_eb0f_d02f_c8bc,
    0x3fd4_a00c_9b0f_3d20, 0x3c78_23ba_6bb0_8ead, 0x3fee_4af1_4b2a_449c, 0xbc86_8ca0_2e8a_6833,
    0x3fd5_190e_cf68_a77a, 0x3c7b_3571_55ee_f0f3, 0x3fee_3614_b680_d6a5, 0xbc72_7793_aa01_5237,
    0x3fd5_91bc_9fa2_f597, 0x3c67_c74b_ac3f_e0cb, 0x3fee_20bf_49ac_d6c1, 0xbc56_60ae_c7ef_636c,
    0x3fd6_0a14_2907_8775, 0x3c5b_1fd8_0ba8_9133, 0x3fee_0af1_5a03_dbce, 0x3c5f_e8e7_0277_1ae6,
    0x3fd6_8213_8a38_d7f7, 0xbc7d_8892_0244_4aad, 0x3fed_f4ab_3ebd_875e, 0xbc8e_2d8a_7e67_36c4,
    0x3fd6_f9b8_e33a_0255, 0x3c74_2bc1_4ee9_da0d, 0x3fed_dded_50f2_28d6, 0xbc6e_80c8_d42b_a2bf,
    0x3fd7_7102_5576_4214, 0xbc66_ead7_314b_b6ce, 0x3fed_c6b7_eb99_5912, 0x3c54_b364_776d_cd35,
    0x3fd7_e7ee_03c8_6d4e, 0xbc7b_63bc_dabf_5af2, 0x3fed_af0b_6b88_8e83, 0x3c8a_249e_2b5e_5cea,
    0x3fd8_5e7a_1282_6949, 0x3c78_a40e_9b5f_ace0, 0x3fed_96e8_2f71_a9dc, 0x3c8f_f61b_d5d2_039d,
    0x3fd8_d4a4_a774_992f, 0x3c74_4a02_ea76_6326, 0x3fed_7e4e_97e1_7b4a, 0xbc63_b770_352b_ed94,
    0x3fd9_4a6b_e9f5_46c5, 0xbc76_9ce1_3e68_3f58, 0x3fed_653f_073e_4040, 0xbc87_6236_434b_ec37,
    0x3fd9_bfce_02e8_0510, 0x3c70_9e39_a320_b0a4, 0x3fed_4bb9_e1c6_19e0, 0x3c8f_34bb_7785_8f61,
    0x3fda_34c9_1cc5_0cca, 0xbc5a_310e_3b50_cecd, 0x3fed_31bf_8d8d_7c06, 0x3c7e_60dd_3089_cbdd,
    0x3fda_a95b_63a0_9277, 0xbc66_293e_b13c_0381, 0x3fed_1750_727d_94f0, 0x3c80_d52b_1ec1_a48e,
    0x3fdb_1d83_0532_1617, 0xbc7a_e242_cb99_f519, 0x3fec_fc6c_fa52_ad9f, 0x3c88_b5b5_508f_2a0d,
    0x3fdb_913e_30db_ac43, 0xbc7e_38ad_2f6c_3ff1, 0x3fec_e115_909a_82e5, 0x3c81_f139_bb31_109a,
    0x3fdc_048b_17b1_40a3, 0x3c61_9fe6_757e_9fa7, 0x3fec_c54a_a2b2_972e, 0x3c64_ee16_2ba8_3a98,
    0x3fdc_7767_ec7f_d19e, 0xbc5e_b14d_1a3d_5826, 0x3fec_a90c_9fc6_7d0b, 0xbc64_6a81_485e_3462,
    0x3fdc_e9d2_e3d4_a51f, 0xbc62_fc8a_12da_e298, 0x3fec_8c5b_f8ce_1a84, 0x3c7a_b3d1_a159_0123,
    0x3fdd_5bca_3404_7661, 0x3c72_8a44_a75f_c29c, 0x3fec_6f39_208b_e53b, 0xbc87_41db_fbaa_db42,
    0x3fdd_cd4c_1532_9c9a, 0x3c70_d4c6_e171_fd9a, 0x3fec_51a4_8b8b_175e, 0xbc61_bbb4_3b9a_a880,
    0x3fde_3e56_c158_2a69, 0xbc50_a482_1099_f88f, 0x3fec_339e_b01d_dd81, 0xbc8c_aaf5_ee82_c5c0,
    0x3fde_aee8_744b_05f0, 0xbc57_89b4_3c9b_027d, 0x3fec_1528_065b_7d50, 0xbc88_9211_1312_e828,
    0x3fdf_1eff_6bc4_f97b, 0x3c71_7212_f8a7_525c, 0x3feb_f641_081e_7536, 0x3c8b_7bd7_1628_a9a1,
    0x3fdf_8e99_e76a_bc97, 0x3c59_d950_af2d_00a3, 0x3feb_d6ea_3102_94f5, 0x3c73_1bbc_c88c_109d,
    0x3fdf_fdb6_28d2_f57a, 0x3c6f_4a99_2e90_5b6a, 0x3feb_b723_fe63_0f32, 0x3c77_2bd2_452d_0a39,
    0x3fe0_3629_39c6_9955, 0xbc82_d8cd_7839_7b01, 0x3feb_96ee_ef58_840e, 0x3c54_5a3c_c78f_ade0,
    0x3fe0_6d36_8694_6e5b, 0x3c83_f5ae_4538_ff1b, 0x3feb_764b_84b7_04c2, 0xbc8f_5848_c21b_389b,
    0x3fe0_a402_1e9e_1001, 0xbc86_f643_a139_14f6, 0x3feb_553a_410c_104e, 0x3c58_ff79_4702_7a16,
    0x3fe0_da8b_26b5_672e, 0xbc8a_58de_f0be_e909, 0x3feb_33bb_a89c_8948, 0x3c8e_a6a5_1d1f_6ca9,
    0x3fe1_10d0_c4b6_9c3b, 0x3c8d_9189_9880_9981, 0x3feb_11d0_4162_a4c6, 0x3c71_dd56_1efb_c0c2,
    0x3fe1_46d2_1f8b_7f82, 0x3c7b_f953_5e27_39a8, 0x3fea_ef78_930b_d275, 0xbc7f_8362_7974_6f94,
    0x3fe1_7c8e_5f2e_edb0, 0x3c63_5e57_102e_2488, 0x3fea_ccb5_26f6_9de5, 0x3c88_fb6a_8dd6_b6cc,
    0x3fe1_b204_acb0_2fdd, 0xbc5f_190c_70cb_b5ff, 0x3fea_a986_8830_8913, 0xbc0b_83d6_07cd_5070,
    0x3fe1_e734_3236_574c, 0x3c72_2a3f_a4f4_1d5a, 0x3fea_85ed_4373_e02d, 0x3c69_be06_385e_c792,
    0x3fe2_1c1c_1b03_94cf, 0x3c5e_5b32_4b23_aa31, 0x3fea_61e9_e725_86af, 0x3c85_8330_e2fd_453f,
    0x3fe2_50bb_9378_8bbb, 0x3c7e_a3d0_2457_bcce, 0x3fea_3d7d_0352_bdcf, 0xbc86_8dba_eca1_9669,
    0x3fe2_8511_c917_a067, 0xbc80_1df1_d9a1_6b70, 0x3fea_18a7_29ae_e445, 0x3c39_5e25_736c_0358,
    0x3fe2_b91d_ea88_421e, 0xbc8f_a371_db21_6ab0, 0x3fe9_f368_ed91_2f85, 0xbc81_d200_c579_1606,
    0x3fe2_ecdf_279a_3082, 0x3c8d_3557_e0e7_e37e, 0x3fe9_cdc2_e3f2_5e5c, 0x3c83_f991_1299_3f62,
    0x3fe3_2054_b148_bc4f, 0x3c8f_6b42_095a_135b, 0x3fe9_a7b5_a36a_6514, 0x3c87_22cf_cc9f_a7a9,
    0x3fe3_537d_b9be_0367, 0x3c6b_327e_7af0_40f0, 0x3fe9_8141_c42e_1310, 0x3c8d_1ff8_0488_f08d,
    0x3fe3_8659_7456_282b, 0xbc71_0fad_a93b_07a8, 0x3fe9_5a67_e00c_b1fd, 0xbc80_befd_a21f_862d,
    0x3fe3_b8e7_15a2_840a, 0xbc79_7653_a7d2_f07b, 0x3fe9_3328_926d_9e92, 0xbc8b_b770_0360_0cda,
    0x3fe3_eb25_d36c_d53a, 0xbc5b_e570_e157_0fc0, 0x3fe9_0b84_784d_daf7, 0xbc70_feb1_0ab9_3b87,
    0x3fe4_1d14_e4ba_6790, 0x3c84_608f_d287_ecf5, 0x3fe8_e37c_303d_9ad1, 0xbc64_63a4_b53d_4bf8,
    0x3fe4_4eb3_81cf_386b, 0xbc83_ed6c_1e6a_5505, 0x3fe8_bb10_5a5d_c900, 0x3c88_63e0_3e94_74c1,
    0x3fe4_8000_e431_159f, 0xbc8b_194a_7463_ed10, 0x3fe8_9241_985d_871f, 0x3c8c_48d9_c413_ed84,
    0x3fe4_b0fc_46aa_b761, 0x3c20_da05_738c_c59a, 0x3fe8_6910_8d77_a6c6, 0x3c73_38ff_e2bf_e9dd,
    0x3fe4_e1a4_e54e_d51b, 0xbc8a_492f_89b7_c76a, 0x3fe8_3f7d_de70_1ca0, 0xbc41_52cf_609b_c6e8,
    0x3fe5_11f9_fd7b_351c, 0xbc85_c0e8_61c4_8831, 0x3fe8_158a_3191_6d5d, 0xbc6d_e8b9_0b82_28de,
    0x3fe5_41fa_cddb_b724, 0x3c72_32c2_8520_d391, 0x3fe7_eb36_2eaa_1488, 0x3c5a_1d65_a4a5_959f,
    0x3fe5_71a6_966d_59b3, 0x3c5c_843b_4d0f_b198, 0x3fe7_c082_7f09_e54f, 0xbc6c_73d6_d72a_ee68,
    0x3fe5_a0fc_9881_3a12, 0xbc8d_82e2_b7d4_227b, 0x3fe7_956f_cd7f_6543, 0xbc8a_b276_e9d4_5ae4,
    0x3fe5_cffc_16bf_8f0d, 0x3c89_6cb3_70eb_578a, 0x3fe7_69fe_c655_211f, 0xbc68_27d5_cf8c_68c5,
    0x3fe5_fea4_552a_9e57, 0x3c80_b6ce_f7ee_20b7, 0x3fe7_3e30_174e_fba1, 0xbc65_d3ae_3d94_ad5f,
    0x3fe6_2cf4_9921_ac79, 0xbc8e_dd98_55b6_241a, 0x3fe7_1204_6fa7_7678, 0x3c84_25b0_a502_9c81,
    0x3fe6_5aec_2963_e755, 0x3c81_26f9_6b71_053c, 0x3fe6_e57c_800c_f55e, 0x3c86_0286_dedb_d0a6,
    0x3fe6_888a_4e13_4b2f, 0xbc86_b7d3_7644_d5e6, 0x3fe6_b898_fa9e_fb5d, 0x3c71_5ac7_86cc_f4b2,
    0x3fe6_b5ce_50b7_821a, 0xbc65_d515_8f70_2e0f, 0x3fe6_8b5a_92eb_6253, 0xbc89_a91a_d985_f89c,
    0x3fe6_e2b7_7c40_bde1, 0xbc70_e729_857f_ad53, 0x3fe6_5dc1_fdeb_8cba, 0xbc59_7c1b_4733_7c77,
    0x3fe7_0f45_1d0a_8c40, 0x3c69_7ede_3885_770d, 0x3fe6_2fcf_f201_91c7, 0x3c6d_9143_8957_56ef,
    0x3fe7_3b76_80de_a578, 0xbc72_2483_06dc_12a2, 0x3fe6_0185_26f5_63df, 0x3c84_6ca5_e0e4_32d0,
    0x3fe7_674a_f6f7_b524, 0x3c7e_9d3f_94ac_84a8, 0x3fe5_d2e2_55f1_f17a, 0x3c80_3141_04c8_892b,
    0x3fe7_92c1_d004_1d52, 0xbc8a_bf05_eeb3_54eb, 0x3fe5_a3e8_3982_4077, 0x3c84_28aa_2759_be62,
    0x3fe7_bdda_5e28_b3c2, 0x3c4a_d119_7ccd_0393, 0x3fe5_7497_8d8e_83f2, 0x3c8f_4714_af28_2d23,
    0x3fe7_e893_f503_7959, 0x3c80_eefb_aa65_0c4c, 0x3fe5_44f1_0f59_2ca5, 0xbc8e_7ae8_e6c7_a62f,
    0x3fe8_12ed_e9ae_4ba4, 0xbc87_830a_df40_2dda, 0x3fe5_14f5_7d7b_f3da, 0x3c74_7a10_8073_c259,
];

/* `static const double toverp[75]` of branred.h: 2/pi in base 2^24.
Every entry is an exact integer below 2^24, so a decimal literal is
exact; machine-generated from the C all the same. */
const TOVERP: [f64; 75] = [
    10680707.0, 7228996.0, 1387004.0, 2578385.0, 16069853.0,
    12639074.0, 9804092.0, 4427841.0, 16666979.0, 11263675.0,
    12935607.0, 2387514.0, 4345298.0, 14681673.0, 3074569.0,
    13734428.0, 16653803.0, 1880361.0, 10960616.0, 8533493.0,
    3062596.0, 8710556.0, 7349940.0, 6258241.0, 3772886.0,
    3769171.0, 3798172.0, 8675211.0, 12450088.0, 3874808.0,
    9961438.0, 366607.0, 15675153.0, 9132554.0, 7151469.0,
    3571407.0, 2607881.0, 12013382.0, 4155038.0, 6285869.0,
    7677882.0, 13102053.0, 15825725.0, 473591.0, 9065106.0,
    15363067.0, 6271263.0, 9264392.0, 5636912.0, 4652155.0,
    7056368.0, 13614112.0, 10155062.0, 1944035.0, 9527646.0,
    15080200.0, 6658437.0, 6231200.0, 6832269.0, 16767104.0,
    5075751.0, 3212806.0, 1398474.0, 7579849.0, 6349435.0,
    12618859.0, 4703257.0, 12806093.0, 14477321.0, 2786137.0,
    12875403.0, 9837734.0, 14528324.0, 13719321.0, 343717.0,
];

/* ==========================================================================
   s_sin.c helpers.  Names and control flow are the C's.
   ========================================================================== */

/// `TAYLOR_SIN(xx, x, dx)` — the macro of `s_sin.c`, i.e.
/// `x - x^3/3! + x^5/5! - … - dx*x^2/2 + dx`.
fn taylor_sin(xx: f64, x: f64, dx: f64) -> f64 {
    /* POLYNOMIAL(xx) = ((((s5*xx + s4)*xx + s3)*xx + s2)*xx) + s1.
    Four .FMA: the trailing `+ s1` consumes POLYNOMIAL2's own multiply. */
    let poly = xx.mul_add(xx.mul_add(xx.mul_add(xx.mul_add(S5, S4), S3), S2), S1);
    /* t = ((POLYNOMIAL (xx) * x - 0.5 * dx) * xx + dx)
    -> _145 = dx * 0.5;  _146 = .FMS (poly, x, _145);  t = .FMA (xx, _146, dx) */
    let t = poly.mul_add(x, -(0.5 * dx)).mul_add(xx, dx);
    /* double res = x + t; */
    x + t
}

/// `SINCOS_TABLE_LOOKUP(u, sn, ssn, cs, ccs)` — `k = u.i[LOW_HALF] << 2`.
///
/// `u` is `big + fabs(x)`, whose low 32 bits are `round(fabs(x) * 128)`;
/// `|x| < 0.85546875` keeps that below 110 and the index inside the table.
#[inline]
fn sincos_table_lookup(u: f64) -> (f64, f64, f64, f64) {
    let k = (((u.to_bits() & 0xffff_ffff) as u32) << 2) as usize;
    (
        f64::from_bits(SINCOSTAB[k]),
        f64::from_bits(SINCOSTAB[k + 1]),
        f64::from_bits(SINCOSTAB[k + 2]),
        f64::from_bits(SINCOSTAB[k + 3]),
    )
}

/// `do_cos (double x, double dx)` — cos of the double-length number
/// `x + dx`, `|x| < 0.855469`, via the table and a short Taylor series.
fn do_cos(x: f64, dx: f64) -> f64 {
    /* if (x < 0) dx = -dx; */
    let dx = if x < 0.0 { -dx } else { dx };

    /* u.x = big + fabs (x);  x = fabs (x) - (u.x - big) + dx; */
    let ax = x.abs();
    let u = BIG + ax;
    let x = (ax - (u - BIG)) + dx;

    let xx = x * x;
    /* s = x + x * xx * (sn3 + xx * sn5)
    -> _73 = x*xx;  _75 = .FMA (xx, sn5, sn3);  s = .FMA (_73, _75, x) */
    let s = (x * xx).mul_add(xx.mul_add(SN5, SN3), x);
    /* c = xx * (cs2 + xx * (cs4 + xx * cs6))
    -> two .FMA, then a bare multiply for the outermost `xx *`. */
    let c = xx * xx.mul_add(xx.mul_add(CS6, CS4), CS2);

    let (sn, ssn, cs, ccs) = sincos_table_lookup(u);

    /* cor = (ccs - s * ssn - cs * c) - sn * s  -> three .FNMA */
    let cor = (-s).mul_add(sn, (-c).mul_add(cs, (-s).mul_add(ssn, ccs)));
    /* return cs + cor; */
    cs + cor
}

/// `do_sin (double x, double dx)` — sin of the double-length number
/// `x + dx`, `|x| < 0.855469`.
fn do_sin(x: f64, dx: f64) -> f64 {
    /* double xold = x;  Max ULP is 0.501 if |x| < 0.126. */
    let xold = x;
    if x.abs() < 0.126 {
        return taylor_sin(x * x, x, dx);
    }

    /* if (x <= 0) dx = -dx; */
    let dx = if x <= 0.0 { -dx } else { dx };

    /* u.x = big + fabs (x);  x = fabs (x) - (u.x - big); */
    let ax = x.abs();
    let u = BIG + ax;
    let x = ax - (u - BIG);

    let xx = x * x;
    /* s = x + (dx + x * xx * (sn3 + xx * sn5))
    -> _35 = x*xx;  _37 = .FMA (xx, sn5, sn3);  _40 = .FMA (_35, _37, dx);
       s = x + _40   (the outer add is *not* fused: _40 is already an .FMA) */
    let s = x + (x * xx).mul_add(xx.mul_add(SN5, SN3), dx);
    /* c = x * dx + xx * (cs2 + xx * (cs4 + xx * cs6))
    -> the leading `x*dx` is the multiply that gets fused into the `+`;
       `xx * (...)` is then left bare. */
    let c = x.mul_add(dx, xx * xx.mul_add(xx.mul_add(CS6, CS4), CS2));

    let (sn, ssn, cs, ccs) = sincos_table_lookup(u);

    /* cor = (ssn + s * ccs - sn * c) + cs * s
    -> _59 = .FMA (s, ccs, ssn);  _61 = .FNMA (c, sn, _59);
       cor = .FMA (s, cs, _61) */
    let cor = s.mul_add(cs, (-c).mul_add(sn, s.mul_add(ccs, ssn)));
    /* return copysign (sn + cor, xold); */
    (sn + cor).copysign(xold)
}

/// `reduce_sincos (double x, double *a, double *da)` — Cody–Waite reduction
/// of `|x| < 105414350` to within pi/2, accurate to 136 bits. Returns
/// `(a, da, n)`.
fn reduce_sincos(x: f64) -> (f64, f64, i32) {
    /* double t = (x * hpinv + toint);  -> .FMA (x, hpinv, toint) */
    let t = x.mul_add(HPINV, TOINT);
    /* double xn = t - toint;  v.x = t; */
    let xn = t - TOINT;
    /* double y = (x - xn * mp1) - xn * mp2
    -> _193 = .FNMA (xn, mp1, x);  y = .FMA (xn, -mp2, _193)
       (GCC folds the sign of the negative constant mp2 into the fma). */
    let y = xn.mul_add(-MP2, (-xn).mul_add(MP1, x));
    /* int4 n = v.i[LOW_HALF] & 3; */
    let n = ((t.to_bits() & 0xffff_ffff) as u32 as i32) & 3;

    /* t1 = xn * pp3;  t2 = y - t1;  db = (y - t2) - t1;
    Both uses of `t1` are subtractions, so both become .FNMA and the
    multiply disappears. */
    let t2 = (-xn).mul_add(PP3, y);
    let db = (-xn).mul_add(PP3, y - t2);

    /* t1 = xn * pp4;  b = t2 - t1;  db += (t2 - b) - t1;  — same shape. */
    let b = (-xn).mul_add(PP4, t2);
    let db = db + (-xn).mul_add(PP4, t2 - b);

    (b, db, n)
}

/// `do_sincos (double a, double da, int4 n)` — sin or cos of `a + da`
/// according to the quadrant `n`.
fn do_sincos(a: f64, da: f64, n: i32) -> f64 {
    let retval = if n & 1 != 0 { do_cos(a, da) } else { do_sin(a, da) };
    if n & 2 != 0 {
        -retval
    } else {
        retval
    }
}

/* ==========================================================================
   branred.c — Payne–Hanek reduction for |x| >= 105414350.

   NOT an FMA build (no branred-fma.c exists), so nothing here is
   contracted: every `a*b + c` below is a rounded multiply followed by a
   rounded add, exactly as the C reads.
   ========================================================================== */

/// One half of `__branred`'s twice-repeated body: accumulate `x_i * 2/pi` in
/// base 2^24 and split the result into `(b, bb, sum)`.
fn branred_half(xi: f64) -> (f64, f64, f64) {
    let mut r = [0.0f64; 6];
    let mut sum = 0.0f64;

    /* u.x = xi;  k = (u.i[HIGH_HALF]>>20)&2047;  k = (k-450)/24;  if (k<0) k=0; */
    let mut k = ((xi.to_bits() >> 52) & 2047) as i32;
    k = (k - 450) / 24; /* C truncating division; all negative results clamp */
    if k < 0 {
        k = 0;
    }

    /* gor.x = t576.x;  gor.i[HIGH_HALF] -= ((k*24)<<20);   i.e. 2^(576-24k),
    an int4 subtraction on the *high word only*, which is exactly a
    decrement of the biased exponent by 24k. */
    let gor_hi = ((T576.to_bits() >> 32) as u32).wrapping_sub(((k * 24) << 20) as u32);
    let mut gor = f64::from_bits(((gor_hi as u64) << 32) | (T576.to_bits() & 0xffff_ffff));

    /* for (i=0;i<6;i++) { r[i] = x1*toverp[k+i]*gor.x; gor.x *= tm24.x; } */
    for i in 0..6 {
        r[i] = xi * TOVERP[(k as usize) + i] * gor;
        gor *= TM24;
    }

    /* for (i=0;i<3;i++) { s=(r[i]+big.x)-big.x; sum+=s; r[i]-=s; } */
    for i in 0..3 {
        let s = (r[i] + BR_BIG) - BR_BIG;
        sum += s;
        r[i] -= s;
    }

    /* t=0; for (i=0;i<6;i++) t+=r[5-i]; */
    let mut t = 0.0f64;
    for i in 0..6 {
        t += r[5 - i];
    }

    /* bb=(((((r[0]-t)+r[1])+r[2])+r[3])+r[4])+r[5]; */
    let mut bb = (((((r[0] - t) + r[1]) + r[2]) + r[3]) + r[4]) + r[5];

    /* s=(t+big.x)-big.x;  sum+=s;  t-=s;  b=t+bb;  bb=(t-b)+bb; */
    let s = (t + BR_BIG) - BR_BIG;
    sum += s;
    t -= s;
    let b = t + bb;
    bb = (t - b) + bb;

    /* s=(sum+big1.x)-big1.x;  sum-=s; */
    let s = (sum + BR_BIG1) - BR_BIG1;
    sum -= s;

    (b, bb, sum)
}

/// `int __branred (double x, double *a, double *aa)` — reduce `x` to
/// `x = n*pi/2 + (a + aa)` with `|a + aa| < pi/4`; returns `(a, aa, n mod 4)`.
fn branred(x: f64) -> (f64, f64, i32) {
    /* x*=tm600.x;  t=x*split;  x1=t-(t-x);  x2=x-x1;   (Veltkamp split) */
    let x = x * TM600;
    let t = x * SPLIT;
    let x1 = t - (t - x);
    let x2 = x - x1;

    let (b1, bb1, sum1) = branred_half(x1);
    let (b2, bb2, sum2) = branred_half(x2);

    /* sum=sum1+sum2;  b=b1+b2;
    bb = (fabs(b1)>fabs(b2))? (b1-b)+b2 : (b2-b)+b1; */
    let mut sum = sum1 + sum2;
    let mut b = b1 + b2;
    let bb = if b1.abs() > b2.abs() { (b1 - b) + b2 } else { (b2 - b) + b1 };

    /* if (b > 0.5) {b-=1.0; sum+=1.0;} else if (b < -0.5) {b+=1.0; sum-=1.0;} */
    if b > 0.5 {
        b -= 1.0;
        sum += 1.0;
    } else if b < -0.5 {
        b += 1.0;
        sum -= 1.0;
    }

    /* s=b+(bb+bb1+bb2);  t=((b-s)+bb)+(bb1+bb2); */
    let s = b + (bb + bb1 + bb2);
    let t = ((b - s) + bb) + (bb1 + bb2);

    /* b=s*split;  t1=b-(b-s);  t2=s-t1;   (second Veltkamp split) */
    let b = s * SPLIT;
    let t1 = b - (b - s);
    let t2 = s - t1;

    /* b=s*hp0.x;
    bb=(((t1*mp1.x-b)+t1*mp2.x)+t2*mp1.x)+(t2*mp2.x+s*hp1.x+t*hp0.x); */
    let b = s * BR_HP0;
    let bb = (((t1 * BR_MP1 - b) + t1 * BR_MP2) + t2 * BR_MP1)
        + (t2 * BR_MP2 + s * BR_HP1 + t * BR_HP0);

    /* s=b+bb;  t=(b-s)+bb;  *a=s;  *aa=t; */
    let s2 = b + bb;
    let t3 = (b - s2) + bb;

    /* return ((int) sum)&3;  — C cast truncates toward zero; |sum| <= 5. */
    (s2, t3, (sum as i32) & 3)
}

/* ==========================================================================
   The two entry points.

   `SET_RESTORE_ROUND_53BIT` is NOT a no-op on x86-64, contrary to what its
   name suggests: glibc's math_private.h defines it as plain
   SET_RESTORE_ROUND on targets that need no i387 precision control, and that
   rewrites the MXCSR rounding-control bits to FE_TONEAREST for the duration
   of the call, restoring them on return. glibc's sin and cos therefore
   answer in round-to-nearest even when their caller is in another rounding
   mode. This port cannot reproduce that — MXCSR is not reachable from safe
   std Rust — so it inherits the ambient mode, and "the default
   floating-point environment" becomes a precondition of this module rather
   than something it enforces. See the Preconditions section of
   `sundials_libm`.

   `__set_errno` affects no returned bit pattern and so does not appear here.
   ========================================================================== */

/// C `sin(double x)` — glibc 2.39 `__sin`.
pub fn sin(x: f64) -> f64 {
    let m = (x.to_bits() >> 32) as u32;
    let k = 0x7fff_ffff & m; /* no sign */

    if k < 0x3e50_0000 {
        /* if x->0 => sin(x)=x  (math_check_force_underflow raises flags only) */
        x
    } else if k < 0x3feb_6000 {
        /*--------------------- 2^-26 < |x| < 0.855469 --------------------*/
        do_sin(x, 0.0)
    } else if k < 0x4003_68fd {
        /*--------------------- 0.855469 < |x| < 2.426265 -----------------*/
        let t = HP0 - x.abs();
        do_cos(t, HP1).copysign(x)
    } else if k < 0x4199_21fb {
        /*--------------------- 2.426265 < |x| < 105414350 ----------------*/
        let (a, da, n) = reduce_sincos(x);
        do_sincos(a, da, n)
    } else if k < 0x7ff0_0000 {
        /*--------------------- 105414350 < |x| < 2^1024 ------------------*/
        let (a, da, n) = branred(x);
        do_sincos(a, da, n)
    } else {
        /*--------------------- |x| > 2^1024 ------------------------------*/
        x / x
    }
}

/// C `cos(double x)` — glibc 2.39 `__cos`.
pub fn cos(x: f64) -> f64 {
    let m = (x.to_bits() >> 32) as u32;
    let k = 0x7fff_ffff & m;

    if k < 0x3e40_0000 {
        /* |x| < 2^-27 => cos(x) = 1 */
        1.0
    } else if k < 0x3feb_6000 {
        /* 2^-27 < |x| < 0.855469 */
        do_cos(x, 0.0)
    } else if k < 0x4003_68fd {
        /* 0.855469 < |x| < 2.426265; 106-bit reduction is enough here. */
        let y = HP0 - x.abs();
        let a = y + HP1;
        let da = (y - a) + HP1;
        do_sin(a, da)
    } else if k < 0x4199_21fb {
        /* 2.426265 < |x| < 105414350 */
        let (a, da, n) = reduce_sincos(x);
        do_sincos(a, da, n + 1)
    } else if k < 0x7ff0_0000 {
        /* 105414350 < |x| < 2^1024 */
        let (a, da, n) = branred(x);
        do_sincos(a, da, n + 1)
    } else {
        x / x
    }
}
