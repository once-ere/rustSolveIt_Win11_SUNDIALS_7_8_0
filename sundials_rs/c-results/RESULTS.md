# c-results — per-variant results

MSVC build, 2026-08-11T12:34:51Z. `same` / `ws-only` / `content` describe this
build's output against the reference `.out` shipped with SUNDIALS 7.8.0,
after removing timing lines from both sides.

| example | argv | dir | build | run | solver error | bytes | vs shipped ref | diff lines | output file |
|---|---|---|---|---|---|---|---|---|---|
| `cvAdvDiff_bnd` |  | cvode/serial | built | ran |  | 869 | same |  | [`cvAdvDiff_bnd.out`](outputs/cvAdvDiff_bnd.out) |
| `cvAnalytic_mels` |  | cvode/serial | built | ran |  | 800 | same |  | [`cvAnalytic_mels.out`](outputs/cvAnalytic_mels.out) |
| `cvDirectDemo_ls` |  | cvode/serial | built | ran |  | 18173 | same |  | [`cvDirectDemo_ls.out`](outputs/cvDirectDemo_ls.out) |
| `cvDisc_dns` |  | cvode/serial | built | ran |  | 3476 | same |  | [`cvDisc_dns.out`](outputs/cvDisc_dns.out) |
| `cvDiurnal_kry_bp` |  | cvode/serial | built | ran |  | 6185 | content | 58 | [`cvDiurnal_kry_bp.out`](outputs/cvDiurnal_kry_bp.out) |
| `cvDiurnal_kry` |  | cvode/serial | built | ran |  | 2923 | content | 42 | [`cvDiurnal_kry.out`](outputs/cvDiurnal_kry.out) |
| `cvKrylovDemo_ls` |  | cvode/serial | built | ran |  | 11980 | content | 218 | [`cvKrylovDemo_ls.out`](outputs/cvKrylovDemo_ls.out) |
| `cvKrylovDemo_ls` | `1` | cvode/serial | built | ran |  | 11980 | content | 218 | [`cvKrylovDemo_ls_1.out`](outputs/cvKrylovDemo_ls_1.out) |
| `cvKrylovDemo_ls` | `2` | cvode/serial | built | ran |  | 11980 | content | 218 | [`cvKrylovDemo_ls_2.out`](outputs/cvKrylovDemo_ls_2.out) |
| `cvKrylovDemo_prec` |  | cvode/serial | built | ran |  | 27116 | same |  | [`cvKrylovDemo_prec.out`](outputs/cvKrylovDemo_prec.out) |
| `cvParticle_dns` |  | cvode/serial | built | ran |  | 911 | content | 16 | [`cvParticle_dns.out`](outputs/cvParticle_dns.out) |
| `cvPendulum_dns` |  | cvode/serial | built | ran |  | 1930 | content | 10 | [`cvPendulum_dns.out`](outputs/cvPendulum_dns.out) |
| `cvRoberts_dns` |  | cvode/serial | built | ran |  | 2267 | same |  | [`cvRoberts_dns.out`](outputs/cvRoberts_dns.out) |
| `cvRoberts_dns_constraints` |  | cvode/serial | built | ran |  | 1285 | same |  | [`cvRoberts_dns_constraints.out`](outputs/cvRoberts_dns_constraints.out) |
| `cvRoberts_dns_negsol` |  | cvode/serial | built | ran |  | 2451 | ws-only | 2 | [`cvRoberts_dns_negsol.out`](outputs/cvRoberts_dns_negsol.out) |
| `cvRoberts_dns_uw` |  | cvode/serial | built | ran |  | 1285 | same |  | [`cvRoberts_dns_uw.out`](outputs/cvRoberts_dns_uw.out) |
| `cvRocket_dns` |  | cvode/serial | built | ran |  | 4292 | same |  | [`cvRocket_dns.out`](outputs/cvRocket_dns.out) |
| `cvVdp_auto_nls` |  | cvode/serial | built | ran |  | 2468 | content | 22 | [`cvVdp_auto_nls.out`](outputs/cvVdp_auto_nls.out) |
| `cvKrylovDemo_ls` | `0 1` | cvode/serial | built | ran |  | 2564 | content | 44 | [`cvKrylovDemo_ls_0_1.out`](outputs/cvKrylovDemo_ls_0_1.out) |
| `cvAdvDiff_bndL` |  | cvode/serial | built via documented LAPACK->native substitution | ran |  | 869 | same |  | [`cvAdvDiff_bndL.out`](outputs/cvAdvDiff_bndL.out) |
| `cvRoberts_dnsL` |  | cvode/serial | built via documented LAPACK->native substitution | ran |  | 1285 | content | 16 | [`cvRoberts_dnsL.out`](outputs/cvRoberts_dnsL.out) |
| `cvRoberts_block_klu` |  | cvode/serial | missing header `klu.h` | excluded(klu) |  | 0 | — |  | — |
| `cvRoberts_klu` |  | cvode/serial | missing header `klu.h` | excluded(klu) |  | 0 | — |  | — |
| `cvRoberts_sps` |  | cvode/serial | missing header `slu_mt_ddefs.h` | excluded(superlu) |  | 0 | — |  | — |
| `cvsAdvDiff_ASAi_bnd` |  | cvodes/serial | built | ran |  | 291 | same |  | [`cvsAdvDiff_ASAi_bnd.out`](outputs/cvsAdvDiff_ASAi_bnd.out) |
| `cvsAdvDiff_FSA_non` | `-sensi sim t` | cvodes/serial | built | ran |  | 3331 | same |  | [`cvsAdvDiff_FSA_non_-sensi_sim_t.out`](outputs/cvsAdvDiff_FSA_non_-sensi_sim_t.out) |
| `cvsAdvDiff_FSA_non` | `-sensi stg t` | cvodes/serial | built | ran |  | 3328 | same |  | [`cvsAdvDiff_FSA_non_-sensi_stg_t.out`](outputs/cvsAdvDiff_FSA_non_-sensi_stg_t.out) |
| `cvsAdvDiff_bnd` |  | cvodes/serial | built | ran |  | 872 | same |  | [`cvsAdvDiff_bnd.out`](outputs/cvsAdvDiff_bnd.out) |
| `cvsAnalytic_mels` |  | cvodes/serial | built | ran |  | 800 | same |  | [`cvsAnalytic_mels.out`](outputs/cvsAnalytic_mels.out) |
| `cvsAnalytic_mels` | `cvodes.max_order 3` | cvodes/serial | built | ran |  | 801 | same |  | [`cvsAnalytic_mels_cvodes.max_order_3.out`](outputs/cvsAnalytic_mels_cvodes.max_order_3.out) |
| `cvsDirectDemo_ls` |  | cvodes/serial | built | ran |  | 18215 | same |  | [`cvsDirectDemo_ls.out`](outputs/cvsDirectDemo_ls.out) |
| `cvsDiurnal_FSA_kry` | `-sensi sim t` | cvodes/serial | built | ran | **YES** | 5778 | content | 104 | [`cvsDiurnal_FSA_kry_-sensi_sim_t.out`](outputs/cvsDiurnal_FSA_kry_-sensi_sim_t.out) |
| `cvsDiurnal_FSA_kry` | `-sensi stg t` | cvodes/serial | built | ran |  | 9083 | content | 92 | [`cvsDiurnal_FSA_kry_-sensi_stg_t.out`](outputs/cvsDiurnal_FSA_kry_-sensi_stg_t.out) |
| `cvsDiurnal_kry` |  | cvodes/serial | built | ran |  | 2923 | content | 42 | [`cvsDiurnal_kry.out`](outputs/cvsDiurnal_kry.out) |
| `cvsDiurnal_kry_bp` |  | cvodes/serial | built | ran |  | 6185 | content | 58 | [`cvsDiurnal_kry_bp.out`](outputs/cvsDiurnal_kry_bp.out) |
| `cvsFoodWeb_ASAi_kry` |  | cvodes/serial | built | ran |  | 1042 | same |  | [`cvsFoodWeb_ASAi_kry.out`](outputs/cvsFoodWeb_ASAi_kry.out) |
| `cvsFoodWeb_ASAp_kry` |  | cvodes/serial | built | ran |  | 1012 | same |  | [`cvsFoodWeb_ASAp_kry.out`](outputs/cvsFoodWeb_ASAp_kry.out) |
| `cvsHessian_ASA_FSA` |  | cvodes/serial | built | ran |  | 2405 | same |  | [`cvsHessian_ASA_FSA.out`](outputs/cvsHessian_ASA_FSA.out) |
| `cvsKrylovDemo_ls` |  | cvodes/serial | built | ran |  | 11980 | content | 253 | [`cvsKrylovDemo_ls.out`](outputs/cvsKrylovDemo_ls.out) |
| `cvsKrylovDemo_ls` | `1` | cvodes/serial | built | ran |  | 11980 | content | 253 | [`cvsKrylovDemo_ls_1.out`](outputs/cvsKrylovDemo_ls_1.out) |
| `cvsKrylovDemo_ls` | `2` | cvodes/serial | built | ran |  | 11980 | content | 253 | [`cvsKrylovDemo_ls_2.out`](outputs/cvsKrylovDemo_ls_2.out) |
| `cvsKrylovDemo_prec` |  | cvodes/serial | built | ran |  | 27117 | same |  | [`cvsKrylovDemo_prec.out`](outputs/cvsKrylovDemo_prec.out) |
| `cvsLotkaVolterra_ASA` |  | cvodes/serial | built | ran |  | 422 | same |  | [`cvsLotkaVolterra_ASA.out`](outputs/cvsLotkaVolterra_ASA.out) |
| `cvsParticle_dns` |  | cvodes/serial | built | ran |  | 911 | content | 16 | [`cvsParticle_dns.out`](outputs/cvsParticle_dns.out) |
| `cvsPendulum_dns` |  | cvodes/serial | built | ran |  | 1930 | content | 10 | [`cvsPendulum_dns.out`](outputs/cvsPendulum_dns.out) |
| `cvsRoberts_ASAi_dns` |  | cvodes/serial | built | ran |  | 5485 | same |  | [`cvsRoberts_ASAi_dns.out`](outputs/cvsRoberts_ASAi_dns.out) |
| `cvsRoberts_ASAi_dns_constraints` |  | cvodes/serial | built | ran |  | 1939 | same |  | [`cvsRoberts_ASAi_dns_constraints.out`](outputs/cvsRoberts_ASAi_dns_constraints.out) |
| `cvsRoberts_FSA_dns` | `-sensi sim t` | cvodes/serial | built | ran |  | 6394 | content | 6 | [`cvsRoberts_FSA_dns_-sensi_sim_t.out`](outputs/cvsRoberts_FSA_dns_-sensi_sim_t.out) |
| `cvsRoberts_FSA_dns` | `-sensi stg1 t` | cvodes/serial | built | ran |  | 6633 | content | 10 | [`cvsRoberts_FSA_dns_-sensi_stg1_t.out`](outputs/cvsRoberts_FSA_dns_-sensi_stg1_t.out) |
| `cvsRoberts_FSA_dns_Switch` |  | cvodes/serial | built | ran |  | 1906 | same |  | [`cvsRoberts_FSA_dns_Switch.out`](outputs/cvsRoberts_FSA_dns_Switch.out) |
| `cvsRoberts_FSA_dns_constraints` | `-sensi stg1 t` | cvodes/serial | built | ran |  | 5364 | same |  | [`cvsRoberts_FSA_dns_constraints_-sensi_stg1_t.out`](outputs/cvsRoberts_FSA_dns_constraints_-sensi_stg1_t.out) |
| `cvsRoberts_dns` |  | cvodes/serial | built | ran |  | 2267 | same |  | [`cvsRoberts_dns.out`](outputs/cvsRoberts_dns.out) |
| `cvsRoberts_dns_constraints` |  | cvodes/serial | built | ran |  | 1285 | same |  | [`cvsRoberts_dns_constraints.out`](outputs/cvsRoberts_dns_constraints.out) |
| `cvsRoberts_dns_uw` |  | cvodes/serial | built | ran |  | 1285 | same |  | [`cvsRoberts_dns_uw.out`](outputs/cvsRoberts_dns_uw.out) |
| `cvsKrylovDemo_ls` | `0 1` | cvodes/serial | built | ran |  | 11980 | content | 798 | [`cvsKrylovDemo_ls_0_1.out`](outputs/cvsKrylovDemo_ls_0_1.out) |
| `cvsAdvDiff_bndL` |  | cvodes/serial | built via documented LAPACK->native substitution | ran |  | 885 | same |  | [`cvsAdvDiff_bndL.out`](outputs/cvsAdvDiff_bndL.out) |
| `cvsRoberts_dnsL` |  | cvodes/serial | built via documented LAPACK->native substitution | ran |  | 1285 | content | 32 | [`cvsRoberts_dnsL.out`](outputs/cvsRoberts_dnsL.out) |
| `cvsRoberts_ASAi_klu` |  | cvodes/serial | missing header `klu.h` | excluded(klu) |  | 0 | — |  | — |
| `cvsRoberts_FSA_klu` | `-sensi stg1 t` | cvodes/serial | missing header `klu.h` | excluded(klu) |  | 0 | — |  | — |
| `cvsRoberts_klu` |  | cvodes/serial | missing header `klu.h` | excluded(klu) |  | 0 | — |  | — |
| `cvsRoberts_ASAi_sps` |  | cvodes/serial | missing header `slu_mt_ddefs.h` | excluded(superlu) |  | 0 | — |  | — |
| `cvsRoberts_FSA_sps` | `-sensi stg1 t` | cvodes/serial | missing header `slu_mt_ddefs.h` | excluded(superlu) |  | 0 | — |  | — |
| `cvsRoberts_sps` |  | cvodes/serial | missing header `slu_mt_ddefs.h` | excluded(superlu) |  | 0 | — |  | — |
| `kinAnalytic_fp` |  | kinsol/serial | built | ran |  | 739 | same |  | [`kinAnalytic_fp.out`](outputs/kinAnalytic_fp.out) |
| `kinAnalytic_fp` | `--damping_fp 0.5` | kinsol/serial | built | ran |  | 748 | same |  | [`kinAnalytic_fp_--damping_fp_0.5.out`](outputs/kinAnalytic_fp_--damping_fp_0.5.out) |
| `kinAnalytic_fp` | `--damping_fn` | kinsol/serial | built | ran |  | 745 | same |  | [`kinAnalytic_fp_--damping_fn.out`](outputs/kinAnalytic_fp_--damping_fn.out) |
| `kinAnalytic_fp` | `--m_aa 2` | kinsol/serial | built | ran |  | 738 | same |  | [`kinAnalytic_fp_--m_aa_2.out`](outputs/kinAnalytic_fp_--m_aa_2.out) |
| `kinAnalytic_fp` | `--m_aa 2 --delay_aa 2` | kinsol/serial | built | ran |  | 739 | same |  | [`kinAnalytic_fp_--m_aa_2_--delay_aa_2.out`](outputs/kinAnalytic_fp_--m_aa_2_--delay_aa_2.out) |
| `kinAnalytic_fp` | `--m_aa 2 --damping_aa 0.5` | kinsol/serial | built | ran |  | 753 | same |  | [`kinAnalytic_fp_--m_aa_2_--damping_aa_0.5.out`](outputs/kinAnalytic_fp_--m_aa_2_--damping_aa_0.5.out) |
| `kinAnalytic_fp` | `--m_aa 2 --damping_fn` | kinsol/serial | built | ran |  | 742 | same |  | [`kinAnalytic_fp_--m_aa_2_--damping_fn.out`](outputs/kinAnalytic_fp_--m_aa_2_--damping_fn.out) |
| `kinAnalytic_fp` | `--m_aa 3 --depth_fn` | kinsol/serial | built | ran |  | 738 | same |  | [`kinAnalytic_fp_--m_aa_3_--depth_fn.out`](outputs/kinAnalytic_fp_--m_aa_3_--depth_fn.out) |
| `kinAnalytic_fp` | `--m_aa 2 --orth_aa 1` | kinsol/serial | built | ran |  | 738 | same |  | [`kinAnalytic_fp_--m_aa_2_--orth_aa_1.out`](outputs/kinAnalytic_fp_--m_aa_2_--orth_aa_1.out) |
| `kinAnalytic_fp` | `--m_aa 2 --orth_aa 2` | kinsol/serial | built | ran |  | 738 | same |  | [`kinAnalytic_fp_--m_aa_2_--orth_aa_2.out`](outputs/kinAnalytic_fp_--m_aa_2_--orth_aa_2.out) |
| `kinAnalytic_fp` | `--m_aa 2 --orth_aa 3` | kinsol/serial | built | ran |  | 738 | same |  | [`kinAnalytic_fp_--m_aa_2_--orth_aa_3.out`](outputs/kinAnalytic_fp_--m_aa_2_--orth_aa_3.out) |
| `kinFerTron_dns` |  | kinsol/serial | built | ran |  | 1574 | same |  | [`kinFerTron_dns.out`](outputs/kinFerTron_dns.out) |
| `kinFoodWeb_kry` |  | kinsol/serial | built | ran |  | 817 | same |  | [`kinFoodWeb_kry.out`](outputs/kinFoodWeb_kry.out) |
| `kinKrylovDemo_ls` |  | kinsol/serial | built | ran |  | 3557 | same |  | [`kinKrylovDemo_ls.out`](outputs/kinKrylovDemo_ls.out) |
| `kinLaplace_bnd` |  | kinsol/serial | built | ran |  | 1848 | same |  | [`kinLaplace_bnd.out`](outputs/kinLaplace_bnd.out) |
| `kinLaplace_picard_bnd` |  | kinsol/serial | built | ran |  | 1792 | same |  | [`kinLaplace_picard_bnd.out`](outputs/kinLaplace_picard_bnd.out) |
| `kinLaplace_picard_kry` |  | kinsol/serial | built | ran |  | 1789 | same |  | [`kinLaplace_picard_kry.out`](outputs/kinLaplace_picard_kry.out) |
| `kinRoberts_fp` |  | kinsol/serial | built | ran |  | 563 | same |  | [`kinRoberts_fp.out`](outputs/kinRoberts_fp.out) |
| `kinRoberts_fp` | `kinsol.m_aa 1` | kinsol/serial | built | ran |  | 563 | same |  | [`kinRoberts_fp_kinsol.m_aa_1.out`](outputs/kinRoberts_fp_kinsol.m_aa_1.out) |
| `kinRoboKin_dns` |  | kinsol/serial | built | ran |  | 1529 | ws-only | 32 | [`kinRoboKin_dns.out`](outputs/kinRoboKin_dns.out) |
| `kinFerTron_klu` |  | kinsol/serial | missing header `klu.h` | excluded(klu) |  | 0 | — |  | — |
| `kinRoboKin_slu` |  | kinsol/serial | missing header `slu_mt_ddefs.h` | excluded(superlu) |  | 0 | — |  | — |
| `idaAnalytic_mels` |  | ida/serial | built | ran |  | 781 | same |  | [`idaAnalytic_mels.out`](outputs/idaAnalytic_mels.out) |
| `idaAnalytic_mels` | `ida.scalar_tolerances 1e-3 1e-8` | ida/serial | built | ran |  | 776 | same |  | [`idaAnalytic_mels_ida.scalar_tolerances_1e-3_1e-8.out`](outputs/idaAnalytic_mels_ida.scalar_tolerances_1e-3_1e-8.out) |
| `idaFoodWeb_bnd` |  | ida/serial | built | ran |  | 1583 | content | 4 | [`idaFoodWeb_bnd.out`](outputs/idaFoodWeb_bnd.out) |
| `idaFoodWeb_kry` |  | ida/serial | built | ran |  | 1558 | content | 4 | [`idaFoodWeb_kry.out`](outputs/idaFoodWeb_kry.out) |
| `idaHeat2D_bnd` |  | ida/serial | built | ran |  | 1495 | same |  | [`idaHeat2D_bnd.out`](outputs/idaHeat2D_bnd.out) |
| `idaHeat2D_kry` |  | ida/serial | built | ran |  | 2701 | same |  | [`idaHeat2D_kry.out`](outputs/idaHeat2D_kry.out) |
| `idaKrylovDemo_ls` |  | ida/serial | built | ran |  | 4865 | same |  | [`idaKrylovDemo_ls.out`](outputs/idaKrylovDemo_ls.out) |
| `idaKrylovDemo_ls` | `1` | ida/serial | built | ran |  | 4865 | same |  | [`idaKrylovDemo_ls_1.out`](outputs/idaKrylovDemo_ls_1.out) |
| `idaKrylovDemo_ls` | `2` | ida/serial | built | ran |  | 4865 | same |  | [`idaKrylovDemo_ls_2.out`](outputs/idaKrylovDemo_ls_2.out) |
| `idaRoberts_dns` |  | ida/serial | built | ran |  | 2743 | same |  | [`idaRoberts_dns.out`](outputs/idaRoberts_dns.out) |
| `idaSlCrank_dns` |  | ida/serial | built | ran |  | 3609 | same |  | [`idaSlCrank_dns.out`](outputs/idaSlCrank_dns.out) |
| `idaHeat2D_klu` |  | ida/serial | missing header `klu.h` | excluded(klu) |  | 0 | — |  | — |
| `idaRoberts_klu` |  | ida/serial | missing header `klu.h` | excluded(klu) |  | 0 | — |  | — |
| `idaRoberts_sps` |  | ida/serial | missing header `slu_mt_ddefs.h` | excluded(superlu) |  | 0 | — |  | — |
| `idasAkzoNob_ASAi_dns` |  | idas/serial | built | ran |  | 585 | content | 3 | [`idasAkzoNob_ASAi_dns.out`](outputs/idasAkzoNob_ASAi_dns.out) |
| `idasAkzoNob_dns` |  | idas/serial | built | ran |  | 3084 | same |  | [`idasAkzoNob_dns.out`](outputs/idasAkzoNob_dns.out) |
| `idasAnalytic_mels` |  | idas/serial | built | ran |  | 870 | same |  | [`idasAnalytic_mels.out`](outputs/idasAnalytic_mels.out) |
| `idasAnalytic_mels` | `idas.init_step 1e-5` | idas/serial | built | ran |  | 870 | same |  | [`idasAnalytic_mels_idas.init_step_1e-5.out`](outputs/idasAnalytic_mels_idas.init_step_1e-5.out) |
| `idasFoodWeb_bnd` |  | idas/serial | built | ran |  | 1584 | content | 4 | [`idasFoodWeb_bnd.out`](outputs/idasFoodWeb_bnd.out) |
| `idasHeat2D_bnd` |  | idas/serial | built | ran |  | 1508 | same |  | [`idasHeat2D_bnd.out`](outputs/idasHeat2D_bnd.out) |
| `idasHeat2D_kry` |  | idas/serial | built | ran |  | 2720 | same |  | [`idasHeat2D_kry.out`](outputs/idasHeat2D_kry.out) |
| `idasHessian_ASA_FSA` |  | idas/serial | built | ran |  | 1300 | same |  | [`idasHessian_ASA_FSA.out`](outputs/idasHessian_ASA_FSA.out) |
| `idasKrylovDemo_ls` |  | idas/serial | built | ran |  | 4922 | same |  | [`idasKrylovDemo_ls.out`](outputs/idasKrylovDemo_ls.out) |
| `idasKrylovDemo_ls` | `1` | idas/serial | built | ran |  | 4922 | same |  | [`idasKrylovDemo_ls_1.out`](outputs/idasKrylovDemo_ls_1.out) |
| `idasKrylovDemo_ls` | `2` | idas/serial | built | ran |  | 4922 | same |  | [`idasKrylovDemo_ls_2.out`](outputs/idasKrylovDemo_ls_2.out) |
| `idasRoberts_ASAi_dns` |  | idas/serial | built | ran |  | 4776 | same |  | [`idasRoberts_ASAi_dns.out`](outputs/idasRoberts_ASAi_dns.out) |
| `idasRoberts_FSA_dns` | `-sensi stg t` | idas/serial | built | ran |  | 7285 | same |  | [`idasRoberts_FSA_dns_-sensi_stg_t.out`](outputs/idasRoberts_FSA_dns_-sensi_stg_t.out) |
| `idasRoberts_dns` |  | idas/serial | built | ran |  | 2751 | same |  | [`idasRoberts_dns.out`](outputs/idasRoberts_dns.out) |
| `idasSlCrank_dns` |  | idas/serial | built | ran |  | 2593 | content | 2 | [`idasSlCrank_dns.out`](outputs/idasSlCrank_dns.out) |
| `idasSlCrank_FSA_dns` |  | idas/serial | built | ran |  | 1065 | content | 18 | [`idasSlCrank_FSA_dns.out`](outputs/idasSlCrank_FSA_dns.out) |
| `idasRoberts_ASAi_klu` |  | idas/serial | missing header `klu.h` | excluded(klu) |  | 0 | — |  | — |
| `idasRoberts_FSA_klu` | `-sensi stg t` | idas/serial | missing header `klu.h` | excluded(klu) |  | 0 | — |  | — |
| `idasRoberts_klu` |  | idas/serial | missing header `klu.h` | excluded(klu) |  | 0 | — |  | — |
| `idasRoberts_ASAi_sps` |  | idas/serial | missing header `slu_mt_ddefs.h` | excluded(superlu) |  | 0 | — |  | — |
| `idasRoberts_FSA_sps` | `-sensi stg t` | idas/serial | missing header `slu_mt_ddefs.h` | excluded(superlu) |  | 0 | — |  | — |
| `idasRoberts_sps` |  | idas/serial | missing header `slu_mt_ddefs.h` | excluded(superlu) |  | 0 | — |  | — |
| `ark_analytic` |  | arkode/C_serial | built | ran |  | 2090 | same |  | [`ark_analytic.out`](outputs/ark_analytic.out) |
| `ark_analytic` | `arkode.scalar_tolerances 1e-6 1e-8 arkode.table_names ARKODE_ESDIRK547L2SA_7_4_5 ARKODE_ERK_NONE` | arkode/C_serial | built | ran |  | 2090 | same |  | [`ark_analytic_arkode.scalar_tolerances_1e-6_1e-8_arkode.table_names_ARKODE_ESDIRK547L2SA_7_4_5_ARKODE_ERK_NONE.out`](outputs/ark_analytic_arkode.scalar_tolerances_1e-6_1e-8_arkode.table_names_ARKODE_ESDIRK547L2SA_7_4_5_ARKODE_ERK_NONE.out) |
| `ark_advection_diffusion_reaction_splitting` |  | arkode/C_serial | built | ran |  | 3390 | same |  | [`ark_advection_diffusion_reaction_splitting.out`](outputs/ark_advection_diffusion_reaction_splitting.out) |
| `ark_analytic_lsrk` |  | arkode/C_serial | built | ran |  | 1183 | content | 18 | [`ark_analytic_lsrk.out`](outputs/ark_analytic_lsrk.out) |
| `ark_analytic_lsrk_varjac` |  | arkode/C_serial | built | ran |  | 1346 | content | 24 | [`ark_analytic_lsrk_varjac.out`](outputs/ark_analytic_lsrk_varjac.out) |
| `ark_analytic_lsrk_domeigest` |  | arkode/C_serial | built | ran |  | 1430 | content | 28 | [`ark_analytic_lsrk_domeigest.out`](outputs/ark_analytic_lsrk_domeigest.out) |
| `ark_analytic_lsrk_domeigest` | `arkid.dom_eig_est_init_preprocess_iters 1 sundomeigestimator.max_iters 1` | arkode/C_serial | built | ran |  | 1418 | content | 28 | [`ark_analytic_lsrk_domeigest_arkid.dom_eig_est_init_preprocess_iters_1_sundomeigestimator.max_iters_1.out`](outputs/ark_analytic_lsrk_domeigest_arkid.dom_eig_est_init_preprocess_iters_1_sundomeigestimator.max_iters_1.out) |
| `ark_analytic_mels` |  | arkode/C_serial | built | ran |  | 831 | same |  | [`ark_analytic_mels.out`](outputs/ark_analytic_mels.out) |
| `ark_analytic_nonlin` |  | arkode/C_serial | built | ran |  | 918 | same |  | [`ark_analytic_nonlin.out`](outputs/ark_analytic_nonlin.out) |
| `ark_analytic_partitioned` | `forcing` | arkode/C_serial | built | ran |  | 1727 | ws-only | 84 | [`ark_analytic_partitioned_forcing.out`](outputs/ark_analytic_partitioned_forcing.out) |
| `ark_analytic_partitioned` | `splitting` | arkode/C_serial | built | ran |  | 1727 | ws-only | 84 | [`ark_analytic_partitioned_splitting.out`](outputs/ark_analytic_partitioned_splitting.out) |
| `ark_analytic_partitioned` | `splitting ARKODE_SPLITTING_BEST_2_2_2` | arkode/C_serial | built | ran |  | 1805 | ws-only | 84 | [`ark_analytic_partitioned_splitting_ARKODE_SPLITTING_BEST_2_2_2.out`](outputs/ark_analytic_partitioned_splitting_ARKODE_SPLITTING_BEST_2_2_2.out) |
| `ark_analytic_partitioned` | `splitting ARKODE_SPLITTING_RUTH_3_3_2` | arkode/C_serial | built | ran |  | 1809 | ws-only | 84 | [`ark_analytic_partitioned_splitting_ARKODE_SPLITTING_RUTH_3_3_2.out`](outputs/ark_analytic_partitioned_splitting_ARKODE_SPLITTING_RUTH_3_3_2.out) |
| `ark_analytic_partitioned` | `splitting ARKODE_SPLITTING_YOSHIDA_8_6_2` | arkode/C_serial | built | ran |  | 1814 | ws-only | 84 | [`ark_analytic_partitioned_splitting_ARKODE_SPLITTING_YOSHIDA_8_6_2.out`](outputs/ark_analytic_partitioned_splitting_ARKODE_SPLITTING_YOSHIDA_8_6_2.out) |
| `ark_analytic_ssprk` |  | arkode/C_serial | built | ran |  | 1026 | content | 6 | [`ark_analytic_ssprk.out`](outputs/ark_analytic_ssprk.out) |
| `ark_brusselator_1D_mri` |  | arkode/C_serial | built | ran |  | 5802 | same |  | [`ark_brusselator_1D_mri.out`](outputs/ark_brusselator_1D_mri.out) |
| `ark_brusselator_fp` |  | arkode/C_serial | built | ran |  | 1134 | same |  | [`ark_brusselator_fp.out`](outputs/ark_brusselator_fp.out) |
| `ark_brusselator_lsrk_domeigest` |  | arkode/C_serial | built | ran |  | 1682 | content | 10 | [`ark_brusselator_lsrk_domeigest.out`](outputs/ark_brusselator_lsrk_domeigest.out) |
| `ark_brusselator_lsrk_externaldomeigest` |  | arkode/C_serial | built | ran |  | 1609 | content | 10 | [`ark_brusselator_lsrk_externaldomeigest.out`](outputs/ark_brusselator_lsrk_externaldomeigest.out) |
| `ark_brusselator_mri` |  | arkode/C_serial | built | ran |  | 1477 | same |  | [`ark_brusselator_mri.out`](outputs/ark_brusselator_mri.out) |
| `ark_brusselator` |  | arkode/C_serial | built | ran |  | 1351 | same |  | [`ark_brusselator.out`](outputs/ark_brusselator.out) |
| `ark_brusselator1D_imexmri` | `0 0.001` | arkode/C_serial | built | ran |  | 1335 | same |  | [`ark_brusselator1D_imexmri_0_0.001.out`](outputs/ark_brusselator1D_imexmri_0_0.001.out) |
| `ark_brusselator1D_imexmri` | `2 0.001` | arkode/C_serial | built | ran |  | 1364 | same |  | [`ark_brusselator1D_imexmri_2_0.001.out`](outputs/ark_brusselator1D_imexmri_2_0.001.out) |
| `ark_brusselator1D_imexmri` | `3 0.001` | arkode/C_serial | built | ran |  | 1462 | same |  | [`ark_brusselator1D_imexmri_3_0.001.out`](outputs/ark_brusselator1D_imexmri_3_0.001.out) |
| `ark_brusselator1D_imexmri` | `4 0.001` | arkode/C_serial | built | ran |  | 1370 | same |  | [`ark_brusselator1D_imexmri_4_0.001.out`](outputs/ark_brusselator1D_imexmri_4_0.001.out) |
| `ark_brusselator1D_imexmri` | `5 0.001` | arkode/C_serial | built | ran |  | 1469 | same |  | [`ark_brusselator1D_imexmri_5_0.001.out`](outputs/ark_brusselator1D_imexmri_5_0.001.out) |
| `ark_brusselator1D_imexmri` | `6 0.001` | arkode/C_serial | built | ran |  | 1372 | same |  | [`ark_brusselator1D_imexmri_6_0.001.out`](outputs/ark_brusselator1D_imexmri_6_0.001.out) |
| `ark_brusselator1D_imexmri` | `7 0.001` | arkode/C_serial | built | ran |  | 1479 | same |  | [`ark_brusselator1D_imexmri_7_0.001.out`](outputs/ark_brusselator1D_imexmri_7_0.001.out) |
| `ark_brusselator1D` |  | arkode/C_serial | built | ran |  | 5786 | same |  | [`ark_brusselator1D.out`](outputs/ark_brusselator1D.out) |
| `ark_conserved_exp_entropy_ark` | `1 0` | arkode/C_serial | built | ran |  | 2987 | content | 48 | [`ark_conserved_exp_entropy_ark_1_0.out`](outputs/ark_conserved_exp_entropy_ark_1_0.out) |
| `ark_conserved_exp_entropy_ark` | `1 1` | arkode/C_serial | built | ran |  | 2251 | content | 31 | [`ark_conserved_exp_entropy_ark_1_1.out`](outputs/ark_conserved_exp_entropy_ark_1_1.out) |
| `ark_conserved_exp_entropy_erk` | `1` | arkode/C_serial | built | ran |  | 2809 | content | 48 | [`ark_conserved_exp_entropy_erk_1.out`](outputs/ark_conserved_exp_entropy_erk_1.out) |
| `ark_damped_harmonic_symplectic` |  | arkode/C_serial | built | ran |  | 995 | ws-only | 26 | [`ark_damped_harmonic_symplectic.out`](outputs/ark_damped_harmonic_symplectic.out) |
| `ark_dissipated_exp_entropy` | `1 0` | arkode/C_serial | built | ran |  | 4301 | content | 82 | [`ark_dissipated_exp_entropy_1_0.out`](outputs/ark_dissipated_exp_entropy_1_0.out) |
| `ark_dissipated_exp_entropy` | `1 1` | arkode/C_serial | built | ran |  | 2989 | content | 1 | [`ark_dissipated_exp_entropy_1_1.out`](outputs/ark_dissipated_exp_entropy_1_1.out) |
| `ark_harmonic_symplectic` |  | arkode/C_serial | built | ran |  | 1284 | content | 28 | [`ark_harmonic_symplectic.out`](outputs/ark_harmonic_symplectic.out) |
| `ark_heat1D_adapt` |  | arkode/C_serial | built | ran |  | 6443 | same |  | [`ark_heat1D_adapt.out`](outputs/ark_heat1D_adapt.out) |
| `ark_heat1D` |  | arkode/C_serial | built | ran |  | 896 | same |  | [`ark_heat1D.out`](outputs/ark_heat1D.out) |
| `ark_kepler` | `--stepper ERK --step-mode adapt` | arkode/C_serial | built | ran |  | 5321 | content | 22 | [`ark_kepler_--stepper_ERK_--step-mode_adapt.out`](outputs/ark_kepler_--stepper_ERK_--step-mode_adapt.out) |
| `ark_kepler` | `--stepper ERK --step-mode fixed --count-orbits` | arkode/C_serial | built | ran |  | 10677 | ws-only | 36 | [`ark_kepler_--stepper_ERK_--step-mode_fixed_--count-orbits.out`](outputs/ark_kepler_--stepper_ERK_--step-mode_fixed_--count-orbits.out) |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --count-orbits --use-compensated-sums` | arkode/C_serial | built | ran |  | 10418 | content | 156 | [`ark_kepler_--stepper_SPRK_--step-mode_fixed_--count-orbits_--use-compensated-sums.out`](outputs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--count-orbits_--use-compensated-sums.out) |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_EULER_1_1 --tf 50 --check-order --nout 1` | arkode/C_serial | built | ran |  | 8570 | ws-only | 242 | [`ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_EULER_1_1_--tf_50_--check-order_--nout_1.out`](outputs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_EULER_1_1_--tf_50_--check-order_--nout_1.out) |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_LEAPFROG_2_2 --tf 50 --check-order --nout 1` | arkode/C_serial | built | ran |  | 8569 | content | 2 | [`ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_LEAPFROG_2_2_--tf_50_--check-order_--nout_1.out`](outputs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_LEAPFROG_2_2_--tf_50_--check-order_--nout_1.out) |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_MCLACHLAN_2_2 --tf 50 --check-order --nout 1` | arkode/C_serial | built | ran |  | 8569 | content | 2 | [`ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_MCLACHLAN_2_2_--tf_50_--check-order_--nout_1.out`](outputs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_MCLACHLAN_2_2_--tf_50_--check-order_--nout_1.out) |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_MCLACHLAN_3_3 --tf 50 --check-order --nout 1` | arkode/C_serial | built | ran |  | 8453 | same |  | [`ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_MCLACHLAN_3_3_--tf_50_--check-order_--nout_1.out`](outputs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_MCLACHLAN_3_3_--tf_50_--check-order_--nout_1.out) |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_MCLACHLAN_4_4 --tf 50 --check-order --nout 1` | arkode/C_serial | built | ran |  | 8462 | same |  | [`ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_MCLACHLAN_4_4_--tf_50_--check-order_--nout_1.out`](outputs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_MCLACHLAN_4_4_--tf_50_--check-order_--nout_1.out) |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_MCLACHLAN_5_6 --tf 50 --check-order --nout 1` | arkode/C_serial | built | ran |  | 8455 | content | 6 | [`ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_MCLACHLAN_5_6_--tf_50_--check-order_--nout_1.out`](outputs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_MCLACHLAN_5_6_--tf_50_--check-order_--nout_1.out) |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_PSEUDO_LEAPFROG_2_2 --tf 50 --check-order --nout 1` | arkode/C_serial | built | ran |  | 8566 | content | 4 | [`ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_PSEUDO_LEAPFROG_2_2_--tf_50_--check-order_--nout_1.out`](outputs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_PSEUDO_LEAPFROG_2_2_--tf_50_--check-order_--nout_1.out) |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_RUTH_3_3 --tf 50 --check-order --nout 1` | arkode/C_serial | built | ran |  | 8457 | ws-only | 242 | [`ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_RUTH_3_3_--tf_50_--check-order_--nout_1.out`](outputs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_RUTH_3_3_--tf_50_--check-order_--nout_1.out) |
| `ark_kepler` | `--stepper SPRK --step-mode fixed --method ARKODE_SPRK_YOSHIDA_6_8 --tf 50 --check-order --nout 1` | arkode/C_serial | built | ran |  | 8460 | content | 2 | [`ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_YOSHIDA_6_8_--tf_50_--check-order_--nout_1.out`](outputs/ark_kepler_--stepper_SPRK_--step-mode_fixed_--method_ARKODE_SPRK_YOSHIDA_6_8_--tf_50_--check-order_--nout_1.out) |
| `ark_kepler` |  | arkode/C_serial | built | ran |  | 5112 | ws-only | 26 | [`ark_kepler.out`](outputs/ark_kepler.out) |
| `ark_kpr_mri` | `0 1 0.005` | arkode/C_serial | built | ran |  | 3659 | content | 4 | [`ark_kpr_mri_0_1_0.005.out`](outputs/ark_kpr_mri_0_1_0.005.out) |
| `ark_kpr_mri` | `1 0 0.01` | arkode/C_serial | built | ran |  | 3528 | same |  | [`ark_kpr_mri_1_0_0.01.out`](outputs/ark_kpr_mri_1_0_0.01.out) |
| `ark_kpr_mri` | `1 1 0.002` | arkode/C_serial | built | ran |  | 3669 | same |  | [`ark_kpr_mri_1_1_0.002.out`](outputs/ark_kpr_mri_1_1_0.002.out) |
| `ark_kpr_mri` | `2 4 0.002` | arkode/C_serial | built | ran |  | 3543 | same |  | [`ark_kpr_mri_2_4_0.002.out`](outputs/ark_kpr_mri_2_4_0.002.out) |
| `ark_kpr_mri` | `3 2 0.001` | arkode/C_serial | built | ran |  | 3550 | same |  | [`ark_kpr_mri_3_2_0.001.out`](outputs/ark_kpr_mri_3_2_0.001.out) |
| `ark_kpr_mri` | `4 3 0.001` | arkode/C_serial | built | ran |  | 3535 | same |  | [`ark_kpr_mri_4_3_0.001.out`](outputs/ark_kpr_mri_4_3_0.001.out) |
| `ark_kpr_mri` | `5 4 0.001` | arkode/C_serial | built | ran |  | 3535 | content | 8 | [`ark_kpr_mri_5_4_0.001.out`](outputs/ark_kpr_mri_5_4_0.001.out) |
| `ark_kpr_mri` | `6 5 0.001` | arkode/C_serial | built | ran |  | 3555 | content | 14 | [`ark_kpr_mri_6_5_0.001.out`](outputs/ark_kpr_mri_6_5_0.001.out) |
| `ark_kpr_mri` | `7 2 0.002` | arkode/C_serial | built | ran |  | 3692 | same |  | [`ark_kpr_mri_7_2_0.002.out`](outputs/ark_kpr_mri_7_2_0.002.out) |
| `ark_kpr_mri` | `8 3 0.001 -100 100 0.5 1` | arkode/C_serial | built | ran |  | 3682 | content | 2 | [`ark_kpr_mri_8_3_0.001_-100_100_0.5_1.out`](outputs/ark_kpr_mri_8_3_0.001_-100_100_0.5_1.out) |
| `ark_kpr_mri` | `9 3 0.001 -100 100 0.5 1` | arkode/C_serial | built | ran |  | 3693 | same |  | [`ark_kpr_mri_9_3_0.001_-100_100_0.5_1.out`](outputs/ark_kpr_mri_9_3_0.001_-100_100_0.5_1.out) |
| `ark_kpr_mri` | `10 4 0.001 -100 100 0.5 1` | arkode/C_serial | built | ran |  | 3692 | content | 26 | [`ark_kpr_mri_10_4_0.001_-100_100_0.5_1.out`](outputs/ark_kpr_mri_10_4_0.001_-100_100_0.5_1.out) |
| `ark_kpr_mri` | `11 2 0.001` | arkode/C_serial | built | ran |  | 3707 | same |  | [`ark_kpr_mri_11_2_0.001.out`](outputs/ark_kpr_mri_11_2_0.001.out) |
| `ark_kpr_mri` | `12 3 0.005` | arkode/C_serial | built | ran |  | 3688 | same |  | [`ark_kpr_mri_12_3_0.005.out`](outputs/ark_kpr_mri_12_3_0.005.out) |
| `ark_kpr_mri` | `13 4 0.01` | arkode/C_serial | built | ran |  | 3685 | same |  | [`ark_kpr_mri_13_4_0.01.out`](outputs/ark_kpr_mri_13_4_0.01.out) |
| `ark_KrylovDemo_prec` |  | arkode/C_serial | built | ran |  | 27453 | same |  | [`ark_KrylovDemo_prec.out`](outputs/ark_KrylovDemo_prec.out) |
| `ark_KrylovDemo_prec` | `1` | arkode/C_serial | built | ran |  | 27453 | same |  | [`ark_KrylovDemo_prec_1.out`](outputs/ark_KrylovDemo_prec_1.out) |
| `ark_KrylovDemo_prec` | `2` | arkode/C_serial | built | ran |  | 27453 | same |  | [`ark_KrylovDemo_prec_2.out`](outputs/ark_KrylovDemo_prec_2.out) |
| `ark_lotka_volterra_ASA` | `--check-freq 1` | arkode/C_serial | built | ran |  | 1247 | same |  | [`ark_lotka_volterra_ASA_--check-freq_1.out`](outputs/ark_lotka_volterra_ASA_--check-freq_1.out) |
| `ark_lotka_volterra_ASA` | `--check-freq 5` | arkode/C_serial | built | ran |  | 1250 | same |  | [`ark_lotka_volterra_ASA_--check-freq_5.out`](outputs/ark_lotka_volterra_ASA_--check-freq_5.out) |
| `ark_onewaycouple_mri` |  | arkode/C_serial | built | ran |  | 1100 | same |  | [`ark_onewaycouple_mri.out`](outputs/ark_onewaycouple_mri.out) |
| `ark_reaction_diffusion_mri` |  | arkode/C_serial | built | ran |  | 2353 | ws-only | 70 | [`ark_reaction_diffusion_mri.out`](outputs/ark_reaction_diffusion_mri.out) |
| `ark_robertson_constraints` |  | arkode/C_serial | built | ran |  | 6416 | same |  | [`ark_robertson_constraints.out`](outputs/ark_robertson_constraints.out) |
| `ark_robertson_root` |  | arkode/C_serial | built | ran |  | 1591 | same |  | [`ark_robertson_root.out`](outputs/ark_robertson_root.out) |
| `ark_robertson` |  | arkode/C_serial | built | ran |  | 6992 | same |  | [`ark_robertson.out`](outputs/ark_robertson.out) |
| `ark_twowaycouple_mri` |  | arkode/C_serial | built | ran |  | 1448 | same |  | [`ark_twowaycouple_mri.out`](outputs/ark_twowaycouple_mri.out) |
| `ark_brusselator_fp` | `1` | arkode/C_serial | built | ran |  | 1134 | same |  | [`ark_brusselator_fp_1.out`](outputs/ark_brusselator_fp_1.out) |
| `ark_brusselator1D_klu` |  | arkode/C_klu | missing header `klu.h` | NO-BINARY |  | 0 | missing |  | [`ark_brusselator1D_klu.out`](outputs/ark_brusselator1D_klu.out) |
| `ark_brusselator1D_manyvec` |  | arkode/C_manyvector | built | ran |  | 5830 | content | 30 | [`ark_brusselator1D_manyvec.out`](outputs/ark_brusselator1D_manyvec.out) |
| `ark_brusselator1D_omp` | `4` | arkode/C_openmp | built | ran |  | 5807 | missing |  | [`ark_brusselator1D_omp_4.out`](outputs/ark_brusselator1D_omp_4.out) |
| `ark_heat1D_omp` | `4` | arkode/C_openmp | built | ran |  | 893 | missing |  | [`ark_heat1D_omp_4.out`](outputs/ark_heat1D_omp_4.out) |
| `ark_analytic_nonlin_ompdev` | `4` | arkode/C_openmpdev | MSVC rejects `#pragma omp target` (OpenMP 4.5 device offload) | NO-BINARY |  | 0 | missing |  | [`ark_analytic_nonlin_ompdev_4.out`](outputs/ark_analytic_nonlin_ompdev_4.out) |
| `ark_heat1D_ompdev` | `4` | arkode/C_openmpdev | MSVC rejects `#pragma omp target` (OpenMP 4.5 device offload) | NO-BINARY |  | 0 | missing |  | [`ark_heat1D_ompdev_4.out`](outputs/ark_heat1D_ompdev_4.out) |
| `ark_heat1D_adapt_ompdev` | `4` | arkode/C_openmpdev | MSVC rejects `#pragma omp target` (OpenMP 4.5 device offload) | NO-BINARY |  | 0 | missing |  | [`ark_heat1D_adapt_ompdev_4.out`](outputs/ark_heat1D_adapt_ompdev_4.out) |
| `ark_diurnal_kry_p` |  | arkode/C_parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`ark_diurnal_kry_p.out`](outputs/ark_diurnal_kry_p.out) |
| `ark_diurnal_kry_bbd_p` |  | arkode/C_parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`ark_diurnal_kry_bbd_p.out`](outputs/ark_diurnal_kry_bbd_p.out) |
| `ark_brusselator1D_task_local_nls` | `--monitor` | arkode/C_parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`ark_brusselator1D_task_local_nls_--monitor.out`](outputs/ark_brusselator1D_task_local_nls_--monitor.out) |
| `ark_brusselator1D_task_local_nls` | `--monitor --global-nls` | arkode/C_parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`ark_brusselator1D_task_local_nls_--monitor_--global-nls.out`](outputs/ark_brusselator1D_task_local_nls_--monitor_--global-nls.out) |
| `ark_brusselator1D_task_local_nls` | `--monitor --explicit --tf 3` | arkode/C_parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`ark_brusselator1D_task_local_nls_--monitor_--explicit_--tf_3.out`](outputs/ark_brusselator1D_task_local_nls_--monitor_--explicit_--tf_3.out) |
| `ark_diurnal_kry_ph` | `1` | arkode/C_parhyp | missing header `HYPRE.h` | NO-BINARY |  | 0 | missing |  | [`ark_diurnal_kry_ph_1.out`](outputs/ark_diurnal_kry_ph_1.out) |
| `ark_petsc_ex25` | `1` | arkode/C_petsc | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`ark_petsc_ex25_1.out`](outputs/ark_petsc_ex25_1.out) |
| `ark_brusselator1D_FEM_slu` |  | arkode/C_superlu-mt | missing header `slu_mt_ddefs.h` | NO-BINARY |  | 0 | missing |  | [`ark_brusselator1D_FEM_slu.out`](outputs/ark_brusselator1D_FEM_slu.out) |
| `cvDiurnal_kry_mpimanyvec` | `2` | cvode/C_mpimanyvector | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cvDiurnal_kry_mpimanyvec_2.out`](outputs/cvDiurnal_kry_mpimanyvec_2.out) |
| `cvAdvDiff_bnd_omp` | `4` | cvode/C_openmp | built | ran |  | 888 | missing |  | [`cvAdvDiff_bnd_omp_4.out`](outputs/cvAdvDiff_bnd_omp_4.out) |
| `cvAdvDiff_kry_ompdev` | `4` | cvode/C_openmpdev | MSVC rejects `#pragma omp target` (OpenMP 4.5 device offload) | NO-BINARY |  | 0 | missing |  | [`cvAdvDiff_kry_ompdev_4.out`](outputs/cvAdvDiff_kry_ompdev_4.out) |
| `cvAdvDiff_diag_p` | `2` | cvode/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cvAdvDiff_diag_p_2.out`](outputs/cvAdvDiff_diag_p_2.out) |
| `cvAdvDiff_non_p` | `2` | cvode/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cvAdvDiff_non_p_2.out`](outputs/cvAdvDiff_non_p_2.out) |
| `cvDiurnal_kry_bbd_p` | `2` | cvode/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cvDiurnal_kry_bbd_p_2.out`](outputs/cvDiurnal_kry_bbd_p_2.out) |
| `cvDiurnal_kry_p` | `2` | cvode/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cvDiurnal_kry_p_2.out`](outputs/cvDiurnal_kry_p_2.out) |
| `cvAdvDiff_non_ph` | `2` | cvode/parhyp | missing header `HYPRE.h` | NO-BINARY |  | 0 | missing |  | [`cvAdvDiff_non_ph_2.out`](outputs/cvAdvDiff_non_ph_2.out) |
| `cv_petsc_ex7` | `1` | cvode/petsc | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cv_petsc_ex7_1.out`](outputs/cv_petsc_ex7_1.out) |
| `cvAdvDiff_petsc` | `1` | cvode/petsc | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cvAdvDiff_petsc_1.out`](outputs/cvAdvDiff_petsc_1.out) |
| `cvsAdvDiff_bnd_omp` | `4` | cvodes/C_openmp | built | ran |  | 888 | missing |  | [`cvsAdvDiff_bnd_omp_4.out`](outputs/cvsAdvDiff_bnd_omp_4.out) |
| `cvsAdvDiff_ASAp_non_p` |  | cvodes/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cvsAdvDiff_ASAp_non_p.out`](outputs/cvsAdvDiff_ASAp_non_p.out) |
| `cvsAdvDiff_FSA_non_p` | `-sensi stg t` | cvodes/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cvsAdvDiff_FSA_non_p_-sensi_stg_t.out`](outputs/cvsAdvDiff_FSA_non_p_-sensi_stg_t.out) |
| `cvsAdvDiff_FSA_non_p` | `-sensi sim t` | cvodes/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cvsAdvDiff_FSA_non_p_-sensi_sim_t.out`](outputs/cvsAdvDiff_FSA_non_p_-sensi_sim_t.out) |
| `cvsAdvDiff_non_p` |  | cvodes/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cvsAdvDiff_non_p.out`](outputs/cvsAdvDiff_non_p.out) |
| `cvsAtmDisp_ASAi_kry_bbd_p` |  | cvodes/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cvsAtmDisp_ASAi_kry_bbd_p.out`](outputs/cvsAtmDisp_ASAi_kry_bbd_p.out) |
| `cvsDiurnal_FSA_kry_p` | `-sensi stg t` | cvodes/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cvsDiurnal_FSA_kry_p_-sensi_stg_t.out`](outputs/cvsDiurnal_FSA_kry_p_-sensi_stg_t.out) |
| `cvsDiurnal_FSA_kry_p` | `-sensi sim t` | cvodes/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cvsDiurnal_FSA_kry_p_-sensi_sim_t.out`](outputs/cvsDiurnal_FSA_kry_p_-sensi_sim_t.out) |
| `cvsDiurnal_kry_bbd_p` |  | cvodes/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cvsDiurnal_kry_bbd_p.out`](outputs/cvsDiurnal_kry_bbd_p.out) |
| `cvsDiurnal_kry_p` |  | cvodes/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`cvsDiurnal_kry_p.out`](outputs/cvsDiurnal_kry_p.out) |
| `idaFoodWeb_bnd_omp` | `4` | ida/C_openmp | built | ran |  | 1569 | missing |  | [`idaFoodWeb_bnd_omp_4.out`](outputs/idaFoodWeb_bnd_omp_4.out) |
| `idaFoodWeb_kry_omp` | `4` | ida/C_openmp | built | ran |  | 1591 | missing |  | [`idaFoodWeb_kry_omp_4.out`](outputs/idaFoodWeb_kry_omp_4.out) |
| `idaFoodWeb_kry_bbd_p` | `1` | ida/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idaFoodWeb_kry_bbd_p_1.out`](outputs/idaFoodWeb_kry_bbd_p_1.out) |
| `idaFoodWeb_kry_p` | `1` | ida/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idaFoodWeb_kry_p_1.out`](outputs/idaFoodWeb_kry_p_1.out) |
| `idaHeat2D_kry_bbd_p` | `1` | ida/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idaHeat2D_kry_bbd_p_1.out`](outputs/idaHeat2D_kry_bbd_p_1.out) |
| `idaHeat2D_kry_p` | `1` | ida/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idaHeat2D_kry_p_1.out`](outputs/idaHeat2D_kry_p_1.out) |
| `idaHeat2D_petsc_spgmr` |  | ida/petsc | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idaHeat2D_petsc_spgmr.out`](outputs/idaHeat2D_petsc_spgmr.out) |
| `idaHeat2D_petsc_snes` |  | ida/petsc | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idaHeat2D_petsc_snes.out`](outputs/idaHeat2D_petsc_snes.out) |
| `idaHeat2D_petsc_snes` | `-pre` | ida/petsc | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idaHeat2D_petsc_snes_-pre.out`](outputs/idaHeat2D_petsc_snes_-pre.out) |
| `idaHeat2D_petsc_snes` | `-jac` | ida/petsc | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idaHeat2D_petsc_snes_-jac.out`](outputs/idaHeat2D_petsc_snes_-jac.out) |
| `idaHeat2D_petsc_snes` | `-jac -pre` | ida/petsc | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idaHeat2D_petsc_snes_-jac_-pre.out`](outputs/idaHeat2D_petsc_snes_-jac_-pre.out) |
| `idasFoodWeb_bnd_omp` | `4` | idas/C_openmp | built | ran |  | 1571 | missing |  | [`idasFoodWeb_bnd_omp_4.out`](outputs/idasFoodWeb_bnd_omp_4.out) |
| `idasFoodWeb_kry_omp` | `4` | idas/C_openmp | built | ran |  | 1593 | missing |  | [`idasFoodWeb_kry_omp_4.out`](outputs/idasFoodWeb_kry_omp_4.out) |
| `idasBruss_ASAp_kry_bbd_p` |  | idas/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idasBruss_ASAp_kry_bbd_p.out`](outputs/idasBruss_ASAp_kry_bbd_p.out) |
| `idasBruss_FSA_kry_bbd_p` |  | idas/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idasBruss_FSA_kry_bbd_p.out`](outputs/idasBruss_FSA_kry_bbd_p.out) |
| `idasBruss_kry_bbd_p` |  | idas/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idasBruss_kry_bbd_p.out`](outputs/idasBruss_kry_bbd_p.out) |
| `idasFoodWeb_kry_bbd_p` |  | idas/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idasFoodWeb_kry_bbd_p.out`](outputs/idasFoodWeb_kry_bbd_p.out) |
| `idasFoodWeb_kry_p` |  | idas/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idasFoodWeb_kry_p.out`](outputs/idasFoodWeb_kry_p.out) |
| `idasHeat2D_FSA_kry_bbd_p` | `-sensi stg t` | idas/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idasHeat2D_FSA_kry_bbd_p_-sensi_stg_t.out`](outputs/idasHeat2D_FSA_kry_bbd_p_-sensi_stg_t.out) |
| `idasHeat2D_kry_bbd_p` |  | idas/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idasHeat2D_kry_bbd_p.out`](outputs/idasHeat2D_kry_bbd_p.out) |
| `idasHeat2D_kry_p` |  | idas/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`idasHeat2D_kry_p.out`](outputs/idasHeat2D_kry_p.out) |
| `kinFoodWeb_kry_omp` | `4` | kinsol/C_openmp | built | ran |  | 839 | missing |  | [`kinFoodWeb_kry_omp_4.out`](outputs/kinFoodWeb_kry_omp_4.out) |
| `kinFoodWeb_kry_bbd_p` | `1` | kinsol/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`kinFoodWeb_kry_bbd_p_1.out`](outputs/kinFoodWeb_kry_bbd_p_1.out) |
| `kinFoodWeb_kry_p` | `1` | kinsol/parallel | missing header `mpi.h` | NO-BINARY |  | 0 | missing |  | [`kinFoodWeb_kry_p_1.out`](outputs/kinFoodWeb_kry_p_1.out) |
