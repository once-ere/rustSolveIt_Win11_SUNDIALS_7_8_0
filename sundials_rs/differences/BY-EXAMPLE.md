# differences — every variant

`C vs Rust`, `C vs ref` and `Rust vs ref` are each one of `same`,
`ws-only`, `content`, after removing timing lines from both sides.
`ref` is the `.out` file shipped with SUNDIALS 7.8.0.

| example | argv | C vs Rust | lines | C vs ref | Rust vs ref | diff |
|---|---|---|---|---|---|---|
| `cvAdvDiff_bnd` |  | same |  | same | same |  |
| `cvAnalytic_mels` |  | same |  | same | same |  |
| `cvDirectDemo_ls` |  | same |  | same | same |  |
| `cvDisc_dns` |  | same |  | same | same |  |
| `cvDiurnal_kry_bp` |  | **content** | 58 | content | same | [diff](diffs/cvDiurnal_kry_bp.out.diff) |
| `cvDiurnal_kry` |  | **content** | 42 | content | same | [diff](diffs/cvDiurnal_kry.out.diff) |
| `cvKrylovDemo_ls` |  | **content** | 218 | content | same | [diff](diffs/cvKrylovDemo_ls.out.diff) |
| `cvKrylovDemo_ls` | `1` | **content** | 218 | content | same | [diff](diffs/cvKrylovDemo_ls_1.out.diff) |
| `cvKrylovDemo_ls` | `2` | **content** | 218 | content | same | [diff](diffs/cvKrylovDemo_ls_2.out.diff) |
| `cvKrylovDemo_prec` |  | same |  | same | same |  |
| `cvParticle_dns` |  | **content** | 16 | content | same | [diff](diffs/cvParticle_dns.out.diff) |
| `cvPendulum_dns` |  | same |  | content | content |  |
| `cvRoberts_dns` |  | same |  | same | same |  |
| `cvRoberts_dns_constraints` |  | same |  | same | same |  |
| `cvRoberts_dns_negsol` |  | same |  | ws-only | ws-only |  |
| `cvRoberts_dns_uw` |  | same |  | same | same |  |
| `cvRocket_dns` |  | same |  | same | same |  |
| `cvVdp_auto_nls` |  | **content** | 22 | content | same | [diff](diffs/cvVdp_auto_nls.out.diff) |
| `cvKrylovDemo_ls` | `0 1` | **content** | 44 | content | same | [diff](diffs/cvKrylovDemo_ls_0_1.out.diff) |
| `cvAdvDiff_bndL` |  | same |  | same | same |  |
| `cvRoberts_dnsL` |  | same |  | content | content |  |
| `cvRoberts_block_klu` |  | — |  | — | — |  |
| `cvRoberts_klu` |  | — |  | — | — |  |
| `cvRoberts_sps` |  | — |  | — | — |  |
| `cvsAdvDiff_ASAi_bnd` |  | same |  | same | same |  |
| `cvsAdvDiff_FSA_non` | `-sensi sim t` | same |  | same | same |  |
| `cvsAdvDiff_FSA_non` | `-sensi stg t` | same |  | same | same |  |
| `cvsAdvDiff_bnd` |  | same |  | same | same |  |
| `cvsAnalytic_mels` |  | same |  | same | same |  |
| `cvsAnalytic_mels` | `cvodes.max_order 3` | same |  | same | same |  |
| `cvsDirectDemo_ls` |  | same |  | same | same |  |
| `cvsDiurnal_FSA_kry` | `-sensi sim t` | **content** | 104 | content | same | [diff](diffs/cvsDiurnal_FSA_kry_-sensi_sim_t.out.diff) |
| `cvsDiurnal_FSA_kry` | `-sensi stg t` | **content** | 92 | content | same | [diff](diffs/cvsDiurnal_FSA_kry_-sensi_stg_t.out.diff) |
| `cvsDiurnal_kry` |  | **content** | 42 | content | same | [diff](diffs/cvsDiurnal_kry.out.diff) |
| `cvsDiurnal_kry_bp` |  | **content** | 58 | content | same | [diff](diffs/cvsDiurnal_kry_bp.out.diff) |
| `cvsFoodWeb_ASAi_kry` |  | same |  | same | same |  |
| `cvsFoodWeb_ASAp_kry` |  | same |  | same | same |  |
| `cvsHessian_ASA_FSA` |  | same |  | same | same |  |
| `cvsKrylovDemo_ls` |  | **content** | 218 | content | content | [diff](diffs/cvsKrylovDemo_ls.out.diff) |
| `cvsKrylovDemo_ls` | `1` | **content** | 218 | content | content | [diff](diffs/cvsKrylovDemo_ls_1.out.diff) |
| `cvsKrylovDemo_ls` | `2` | **content** | 218 | content | content | [diff](diffs/cvsKrylovDemo_ls_2.out.diff) |
| `cvsKrylovDemo_prec` |  | same |  | same | same |  |
| `cvsLotkaVolterra_ASA` |  | same |  | same | same |  |
| `cvsParticle_dns` |  | **content** | 16 | content | same | [diff](diffs/cvsParticle_dns.out.diff) |
| `cvsPendulum_dns` |  | same |  | content | content |  |
| `cvsRoberts_ASAi_dns` |  | same |  | same | same |  |
| `cvsRoberts_ASAi_dns_constraints` |  | same |  | same | same |  |
| `cvsRoberts_FSA_dns` | `-sensi sim t` | **content** | 6 | content | same | [diff](diffs/cvsRoberts_FSA_dns_-sensi_sim_t.out.diff) |
| `cvsRoberts_FSA_dns` | `-sensi stg1 t` | **content** | 10 | content | same | [diff](diffs/cvsRoberts_FSA_dns_-sensi_stg1_t.out.diff) |
| `cvsRoberts_FSA_dns_Switch` |  | same |  | same | same |  |
| `cvsRoberts_FSA_dns_constraints` | `-sensi stg1 t` | same |  | same | same |  |
| `cvsRoberts_dns` |  | same |  | same | same |  |
| `cvsRoberts_dns_constraints` |  | same |  | same | same |  |
| `cvsRoberts_dns_uw` |  | same |  | same | same |  |
| `cvsKrylovDemo_ls` | `0 1` | **content** | 218 | content | content | [diff](diffs/cvsKrylovDemo_ls_0_1.out.diff) |
| `cvsAdvDiff_bndL` |  | same |  | same | same |  |
| `cvsRoberts_dnsL` |  | same |  | content | content |  |
| `cvsRoberts_ASAi_klu` |  | — |  | — | — |  |
| `cvsRoberts_FSA_klu` | `-sensi stg1 t` | — |  | — | — |  |
| `cvsRoberts_klu` |  | — |  | — | — |  |
| `cvsRoberts_ASAi_sps` |  | — |  | — | — |  |
| `cvsRoberts_FSA_sps` | `-sensi stg1 t` | — |  | — | — |  |
| `cvsRoberts_sps` |  | — |  | — | — |  |
| `kinAnalytic_fp` |  | same |  | same | same |  |
| `kinAnalytic_fp` | `--damping_fp 0.5` | same |  | same | same |  |
| `kinAnalytic_fp` | `--damping_fn` | same |  | same | same |  |
| `kinAnalytic_fp` | `--m_aa 2` | same |  | same | same |  |
| `kinAnalytic_fp` | `--m_aa 2 --delay_aa 2` | same |  | same | same |  |
| `kinAnalytic_fp` | `--m_aa 2 --damping_aa 0.5` | same |  | same | same |  |
| `kinAnalytic_fp` | `--m_aa 2 --damping_fn` | same |  | same | same |  |
| `kinAnalytic_fp` | `--m_aa 3 --depth_fn` | same |  | same | same |  |
| `kinAnalytic_fp` | `--m_aa 2 --orth_aa 1` | same |  | same | same |  |
| `kinAnalytic_fp` | `--m_aa 2 --orth_aa 2` | same |  | same | same |  |
| `kinAnalytic_fp` | `--m_aa 2 --orth_aa 3` | same |  | same | same |  |
| `kinFerTron_dns` |  | same |  | same | same |  |
| `kinFoodWeb_kry` |  | same |  | same | same |  |
| `kinKrylovDemo_ls` |  | same |  | same | same |  |
| `kinLaplace_bnd` |  | same |  | same | same |  |
| `kinLaplace_picard_bnd` |  | same |  | same | same |  |
| `kinLaplace_picard_kry` |  | same |  | same | same |  |
| `kinRoberts_fp` |  | same |  | same | same |  |
| `kinRoberts_fp` | `kinsol.m_aa 1` | same |  | same | same |  |
| `kinRoboKin_dns` |  | same |  | ws-only | ws-only |  |
| `kinFerTron_klu` |  | — |  | — | — |  |
| `kinRoboKin_slu` |  | — |  | — | — |  |
| `idaAnalytic_mels` |  | same |  | same | same |  |
| `idaAnalytic_mels` | `ida.scalar_tolerances 1e-3 1e-8` | same |  | same | same |  |
| `idaFoodWeb_bnd` |  | **content** | 4 | content | same | [diff](diffs/idaFoodWeb_bnd.out.diff) |
| `idaFoodWeb_kry` |  | **content** | 4 | content | same | [diff](diffs/idaFoodWeb_kry.out.diff) |
| `idaHeat2D_bnd` |  | same |  | same | same |  |
| `idaHeat2D_kry` |  | same |  | same | same |  |
| `idaKrylovDemo_ls` |  | same |  | same | same |  |
| `idaKrylovDemo_ls` | `1` | same |  | same | same |  |
| `idaKrylovDemo_ls` | `2` | same |  | same | same |  |
| `idaRoberts_dns` |  | same |  | same | same |  |
| `idaSlCrank_dns` |  | same |  | same | same |  |
| `idaHeat2D_klu` |  | — |  | — | — |  |
| `idaRoberts_klu` |  | — |  | — | — |  |
| `idaRoberts_sps` |  | — |  | — | — |  |
| `idasAkzoNob_ASAi_dns` |  | same |  | content | content |  |
| `idasAkzoNob_dns` |  | same |  | same | same |  |
| `idasAnalytic_mels` |  | same |  | same | same |  |
| `idasAnalytic_mels` | `idas.init_step 1e-5` | same |  | same | same |  |
| `idasFoodWeb_bnd` |  | **content** | 4 | content | same | [diff](diffs/idasFoodWeb_bnd.out.diff) |
| `idasHeat2D_bnd` |  | same |  | same | same |  |
| `idasHeat2D_kry` |  | same |  | same | same |  |
| `idasHessian_ASA_FSA` |  | same |  | same | same |  |
| `idasKrylovDemo_ls` |  | same |  | same | same |  |
| `idasKrylovDemo_ls` | `1` | same |  | same | same |  |
| `idasKrylovDemo_ls` | `2` | same |  | same | same |  |
| `idasRoberts_ASAi_dns` |  | same |  | same | same |  |
| `idasRoberts_FSA_dns` | `-sensi stg t` | same |  | same | same |  |
| `idasRoberts_dns` |  | same |  | same | same |  |
| `idasSlCrank_dns` |  | **content** | 2 | content | same | [diff](diffs/idasSlCrank_dns.out.diff) |
| `idasSlCrank_FSA_dns` |  | **content** | 18 | content | same | [diff](diffs/idasSlCrank_FSA_dns.out.diff) |
| `idasRoberts_ASAi_klu` |  | — |  | — | — |  |
| `idasRoberts_FSA_klu` | `-sensi stg t` | — |  | — | — |  |
| `idasRoberts_klu` |  | — |  | — | — |  |
| `idasRoberts_ASAi_sps` |  | — |  | — | — |  |
| `idasRoberts_FSA_sps` | `-sensi stg t` | — |  | — | — |  |
| `idasRoberts_sps` |  | — |  | — | — |  |
| `ark_analytic` |  | same |  | same | same |  |
| `ark_analytic` | `arkode.scalar_tolerances 1e-6 1e-8 arkode.table_names ARKODE_ESDIRK547L2SA_7_4_5 ARKODE_ERK_NONE` | same |  | same | same |  |
| `ark_advection_diffusion_reaction_splitting` |  | same |  | same | same |  |
| `ark_analytic_lsrk` |  | **content** | 18 | content | same | [diff](diffs/ark_analytic_lsrk.out.diff) |
| `ark_analytic_lsrk_varjac` |  | **content** | 24 | content | same | [diff](diffs/ark_analytic_lsrk_varjac.out.diff) |
| `ark_analytic_lsrk_domeigest` |  | **content** | 28 | content | same | [diff](diffs/ark_analytic_lsrk_domeigest.out.diff) |
| `ark_analytic_lsrk_domeigest` | `arkid.dom_eig_est_init_preprocess_iters 1 sundomeigestimator.max_iters 1` | **content** | 28 | content | same | [diff](diffs/ark_analytic_lsrk_domeigest_arkid.dom_eig_est_init_preprocess_iters_1_sundomeigestimator.max_iters_1.out.diff) |
| `ark_analytic_mels` |  | same |  | same | same |  |
| `ark_analytic_nonlin` |  | same |  | same | same |  |
| `ark_analytic_partitioned` | `forcing` | same |  | ws-only | ws-only |  |
| `ark_analytic_partitioned` | `splitting` | same |  | ws-only | ws-only |  |
| `ark_analytic_partitioned` | `splitting ARKODE_SPLITTING_BEST_2_2_2` | same |  | ws-only | ws-only |  |
| `ark_analytic_partitioned` | `splitting ARKODE_SPLITTING_RUTH_3_3_2` | same |  | ws-only | ws-only |  |
| `ark_analytic_partitioned` | `splitting ARKODE_SPLITTING_YOSHIDA_8_6_2` | same |  | ws-only | ws-only |  |
| `ark_analytic_ssprk` |  | **content** | 6 | content | same | [diff](diffs/ark_analytic_ssprk.out.diff) |
| `ark_brusselator_1D_mri` |  | same |  | same | same |  |
| `ark_brusselator_fp` |  | same |  | same | same |  |
| `ark_brusselator_lsrk_domeigest` |  | **content** | 10 | content | same | [diff](diffs/ark_brusselator_lsrk_domeigest.out.diff) |
| `ark_brusselator_lsrk_externaldomeigest` |  | **content** | 10 | content | same | [diff](diffs/ark_brusselator_lsrk_externaldomeigest.out.diff) |
| `ark_brusselator_mri` |  | same |  | same | same |  |
| `ark_brusselator` |  | same |  | same | same |  |
| `ark_brusselator1D_imexmri` | `0 0.001` | same |  | same | same |  |
| `ark_brusselator1D_imexmri` | `2 0.001` | same |  | same | same |  |
| `ark_brusselator1D_imexmri` | `3 0.001` | same |  | same | same |  |
| `ark_brusselator1D_imexmri` | `4 0.001` | same |  | same | same |  |
| `ark_brusselator1D_imexmri` | `5 0.001` | same |  | same | same |  |
| `ark_brusselator1D_imexmri` | `6 0.001` | same |  | same | same |  |
| `ark_brusselator1D_imexmri` | `7 0.001` | same |  | same | same |  |
| `ark_brusselator1D` |  | same |  | same | same |  |
| `ark_conserved_exp_entropy_ark` | `1 0` | **content** | 48 | content | same | [diff](diffs/ark_conserved_exp_entropy_ark_1_0.out.diff) |
| `ark_conserved_exp_entropy_ark` | `1 1` | **content** | 30 | content | content | [diff](diffs/ark_conserved_exp_entropy_ark_1_1.out.diff) |
| `ark_conserved_exp_entropy_erk` | `1` | **content** | 48 | content | same | [diff](diffs/ark_conserved_exp_entropy_erk_1.out.diff) |
| `ark_damped_harmonic_symplectic` |  | same |  | ws-only | ws-only |  |
| `ark_dissipated_exp_entropy` | `1 0` | **content** | 82 | content | same | [diff](diffs/ark_dissipated_exp_entropy_1_0.out.diff) |
| `ark_dissipated_exp_entropy` | `1 1` | same |  | content | content |  |
| `ark_harmonic_symplectic` |  | **content** | 2 | content | ws-only | [diff](diffs/ark_harmonic_symplectic.out.diff) |
| `ark_heat1D_adapt` |  | same |  | same | same |  |
| `ark_heat1D` |  | same |  | same | same |  |
| `ark_kepler` | `--stepper ERK --step-mode adapt` | **content** | 22 | content | same | [diff](diffs/ark_kepler_--stepper_ERK_--step-mode_adapt.out.diff) |
| `ark_kepler` | `--stepper ERK --step-mode fixed --count-orbits` | same |  | ws-only | ws-only |  |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --count-orbits --use-compensated-sums` | **content** | 130 | content | ws-only | [diff](diffs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--count-orbits_--use-compensated-sums.out.diff) |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_EULER_1_1 --tf 50 --check-order --nout 1` | same |  | ws-only | ws-only |  |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_LEAPFROG_2_2 --tf 50 --check-order --nout 1` | **content** | 2 | content | same | [diff](diffs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_LEAPFROG_2_2_--tf_50_--check-order_--nout_1.out.diff) |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_MCLACHLAN_2_2 --tf 50 --check-order --nout 1` | **content** | 2 | content | same | [diff](diffs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_MCLACHLAN_2_2_--tf_50_--check-order_--nout_1.out.diff) |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_MCLACHLAN_3_3 --tf 50 --check-order --nout 1` | same |  | same | same |  |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_MCLACHLAN_4_4 --tf 50 --check-order --nout 1` | same |  | same | same |  |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_MCLACHLAN_5_6 --tf 50 --check-order --nout 1` | **content** | 6 | content | same | [diff](diffs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_MCLACHLAN_5_6_--tf_50_--check-order_--nout_1.out.diff) |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_PSEUDO_LEAPFROG_2_2 --tf 50 --check-order --nout 1` | **content** | 4 | content | same | [diff](diffs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_PSEUDO_LEAPFROG_2_2_--tf_50_--check-order_--nout_1.out.diff) |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_RUTH_3_3 --tf 50 --check-order --nout 1` | same |  | ws-only | ws-only |  |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_YOSHIDA_6_8 --tf 50 --check-order --nout 1` | **content** | 2 | content | same | [diff](diffs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_YOSHIDA_6_8_--tf_50_--check-order_--nout_1.out.diff) |
| `ark_kepler` |  | same |  | ws-only | ws-only |  |
| `ark_kpr_mri` | `0 1 0.005` | **content** | 4 | content | same | [diff](diffs/ark_kpr_mri_0_1_0.005.out.diff) |
| `ark_kpr_mri` | `1 0 0.01` | same |  | same | same |  |
| `ark_kpr_mri` | `1 1 0.002` | same |  | same | same |  |
| `ark_kpr_mri` | `2 4 0.002` | same |  | same | same |  |
| `ark_kpr_mri` | `3 2 0.001` | same |  | same | same |  |
| `ark_kpr_mri` | `4 3 0.001` | same |  | same | same |  |
| `ark_kpr_mri` | `5 4 0.001` | **content** | 8 | content | same | [diff](diffs/ark_kpr_mri_5_4_0.001.out.diff) |
| `ark_kpr_mri` | `6 5 0.001` | **content** | 14 | content | same | [diff](diffs/ark_kpr_mri_6_5_0.001.out.diff) |
| `ark_kpr_mri` | `7 2 0.002` | same |  | same | same |  |
| `ark_kpr_mri` | `8 3 0.001 -100 100 0.5 1` | **content** | 2 | content | same | [diff](diffs/ark_kpr_mri_8_3_0.001_-100_100_0.5_1.out.diff) |
| `ark_kpr_mri` | `9 3 0.001 -100 100 0.5 1` | same |  | same | same |  |
| `ark_kpr_mri` | `10 4 0.001 -100 100 0.5 1` | **content** | 26 | content | same | [diff](diffs/ark_kpr_mri_10_4_0.001_-100_100_0.5_1.out.diff) |
| `ark_kpr_mri` | `11 2 0.001` | same |  | same | same |  |
| `ark_kpr_mri` | `12 3 0.005` | same |  | same | same |  |
| `ark_kpr_mri` | `13 4 0.01` | same |  | same | same |  |
| `ark_KrylovDemo_prec` |  | same |  | same | same |  |
| `ark_KrylovDemo_prec` | `1` | same |  | same | same |  |
| `ark_KrylovDemo_prec` | `2` | same |  | same | same |  |
| `ark_lotka_volterra_ASA` | `--check-freq 1` | same |  | same | same |  |
| `ark_lotka_volterra_ASA` | `--check-freq 5` | same |  | same | same |  |
| `ark_onewaycouple_mri` |  | same |  | same | same |  |
| `ark_reaction_diffusion_mri` |  | same |  | ws-only | ws-only |  |
| `ark_robertson_constraints` |  | same |  | same | same |  |
| `ark_robertson_root` |  | same |  | same | same |  |
| `ark_robertson` |  | same |  | same | same |  |
| `ark_twowaycouple_mri` |  | same |  | same | same |  |
| `ark_brusselator_fp` | `1` | same |  | same | same |  |
| `ark_brusselator1D_klu` |  | **missing** |  | missing | missing |  |
| `ark_brusselator1D_manyvec` |  | **missing** |  | content | missing |  |
| `ark_brusselator1D_omp` | `4` | **missing** |  | missing | missing |  |
| `ark_heat1D_omp` | `4` | **missing** |  | missing | missing |  |
| `ark_analytic_nonlin_ompdev` | `4` | **missing** |  | missing | missing |  |
| `ark_heat1D_ompdev` | `4` | **missing** |  | missing | missing |  |
| `ark_heat1D_adapt_ompdev` | `4` | **missing** |  | missing | missing |  |
| `ark_diurnal_kry_p` |  | **missing** |  | missing | missing |  |
| `ark_diurnal_kry_bbd_p` |  | **missing** |  | missing | missing |  |
| `ark_brusselator1D_task_local_nls` | `--monitor` | **missing** |  | missing | missing |  |
| `ark_brusselator1D_task_local_nls` | `--monitor --global-nls` | **missing** |  | missing | missing |  |
| `ark_brusselator1D_task_local_nls` | `--monitor --explicit --tf 3` | **missing** |  | missing | missing |  |
| `ark_diurnal_kry_ph` | `1` | **missing** |  | missing | missing |  |
| `ark_petsc_ex25` | `1` | **missing** |  | missing | missing |  |
| `ark_brusselator1D_FEM_slu` |  | **missing** |  | missing | missing |  |
| `cvDiurnal_kry_mpimanyvec` | `2` | **missing** |  | missing | missing |  |
| `cvAdvDiff_bnd_omp` | `4` | **missing** |  | missing | missing |  |
| `cvAdvDiff_kry_ompdev` | `4` | **missing** |  | missing | missing |  |
| `cvAdvDiff_diag_p` | `2` | **missing** |  | missing | missing |  |
| `cvAdvDiff_non_p` | `2` | **missing** |  | missing | missing |  |
| `cvDiurnal_kry_bbd_p` | `2` | **missing** |  | missing | missing |  |
| `cvDiurnal_kry_p` | `2` | **missing** |  | missing | missing |  |
| `cvAdvDiff_non_ph` | `2` | **missing** |  | missing | missing |  |
| `cv_petsc_ex7` | `1` | **missing** |  | missing | missing |  |
| `cvAdvDiff_petsc` | `1` | **missing** |  | missing | missing |  |
| `cvsAdvDiff_bnd_omp` | `4` | **missing** |  | missing | missing |  |
| `cvsAdvDiff_ASAp_non_p` |  | **missing** |  | missing | missing |  |
| `cvsAdvDiff_FSA_non_p` | `-sensi stg t` | **missing** |  | missing | missing |  |
| `cvsAdvDiff_FSA_non_p` | `-sensi sim t` | **missing** |  | missing | missing |  |
| `cvsAdvDiff_non_p` |  | **missing** |  | missing | missing |  |
| `cvsAtmDisp_ASAi_kry_bbd_p` |  | **missing** |  | missing | missing |  |
| `cvsDiurnal_FSA_kry_p` | `-sensi stg t` | **missing** |  | missing | missing |  |
| `cvsDiurnal_FSA_kry_p` | `-sensi sim t` | **missing** |  | missing | missing |  |
| `cvsDiurnal_kry_bbd_p` |  | **missing** |  | missing | missing |  |
| `cvsDiurnal_kry_p` |  | **missing** |  | missing | missing |  |
| `idaFoodWeb_bnd_omp` | `4` | **missing** |  | missing | missing |  |
| `idaFoodWeb_kry_omp` | `4` | **missing** |  | missing | missing |  |
| `idaFoodWeb_kry_bbd_p` | `1` | **missing** |  | missing | missing |  |
| `idaFoodWeb_kry_p` | `1` | **missing** |  | missing | missing |  |
| `idaHeat2D_kry_bbd_p` | `1` | **missing** |  | missing | missing |  |
| `idaHeat2D_kry_p` | `1` | **missing** |  | missing | missing |  |
| `idaHeat2D_petsc_spgmr` |  | **missing** |  | missing | missing |  |
| `idaHeat2D_petsc_snes` |  | **missing** |  | missing | missing |  |
| `idaHeat2D_petsc_snes` | `-pre` | **missing** |  | missing | missing |  |
| `idaHeat2D_petsc_snes` | `-jac` | **missing** |  | missing | missing |  |
| `idaHeat2D_petsc_snes` | `-jac -pre` | **missing** |  | missing | missing |  |
| `idasFoodWeb_bnd_omp` | `4` | **missing** |  | missing | missing |  |
| `idasFoodWeb_kry_omp` | `4` | **missing** |  | missing | missing |  |
| `idasBruss_ASAp_kry_bbd_p` |  | **missing** |  | missing | missing |  |
| `idasBruss_FSA_kry_bbd_p` |  | **missing** |  | missing | missing |  |
| `idasBruss_kry_bbd_p` |  | **missing** |  | missing | missing |  |
| `idasFoodWeb_kry_bbd_p` |  | **missing** |  | missing | missing |  |
| `idasFoodWeb_kry_p` |  | **missing** |  | missing | missing |  |
| `idasHeat2D_FSA_kry_bbd_p` | `-sensi stg t` | **missing** |  | missing | missing |  |
| `idasHeat2D_kry_bbd_p` |  | **missing** |  | missing | missing |  |
| `idasHeat2D_kry_p` |  | **missing** |  | missing | missing |  |
| `kinFoodWeb_kry_omp` | `4` | **missing** |  | missing | missing |  |
| `kinFoodWeb_kry_bbd_p` | `1` | **missing** |  | missing | missing |  |
| `kinFoodWeb_kry_p` | `1` | **missing** |  | missing | missing |  |
