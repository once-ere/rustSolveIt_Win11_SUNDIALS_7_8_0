# reboundx_rust (`reboundx_rs`)

A pure-Rust translation of **[REBOUNDx](https://github.com/dtamayo/reboundx)
5.1.0** — "REBOUND eXtras", the library by **Dan Tamayo**, Hanno Rein and
collaborators that adds extra physics to a
[REBOUND](https://github.com/hannorein/rebound) N-body simulation.

It is the companion of [`rebound_rs`](../rebound_rust), the pure-Rust
translation of REBOUND itself, and depends on it exactly as the C
`libreboundx` links against `librebound`.

- Zero `unsafe`, zero external dependencies (std only), zero build warnings.
- C function and struct names preserved (`rebx_attach`, `rebx_add_force`,
  `rebx_set_param_double`, `rebx_tides_spin`, ...).
- Verified against the MSVC-compiled C REBOUNDx on the same machine; see
  `../rebound_rust/reboundx_port_test.md` for the acceptance tests and
  `../rebound_rust/rebound_rust.md` for the complete provenance.

**All of the science, and all of the credit, belongs to the original authors.**

## What it gives you

Effects you can switch on in a simulation:

| Kind | Effects |
|---|---|
| Relativity | `gr`, `gr_full`, `gr_potential`, `lense_thirring` |
| Tides & spin | `tides_spin`, `tides_constant_time_lag`, `tides_dynamical` |
| Migration & disks | `modify_orbits_forces`, `modify_orbits_direct`, `type_I_migration`, `exponential_migration`, `inner_disk_edge`, `gas_damping_timescale`, `gas_dynamical_friction` |
| Radiation | `radiation_forces`, `yarkovsky_effect` |
| Gravity shape | `gravitational_harmonics` (J2, J4), `central_force` |
| Other | `stochastic_forces`, `modify_mass`, `track_min_distance`, `integrate_force` |

Plus REBOUNDx's own sub-integrators (`euler`, `rk2`, `rk4`,
`implicit_midpoint`), parameter interpolation, and operator steppers.

## Quick start

```toml
[dependencies]
rebound_rs  = { path = "../rebound_rust" }
reboundx_rs = { path = "." }
```

```rust
use rebound_rs::*;
use reboundx_rs::*;

let mut sim = reb_simulation_create();
// ... add particles ...

rebx_attach(&mut sim);
let gr = rebx_load_force(&mut sim, "gr_potential").unwrap();
rebx_add_force(&mut sim, gr);
if let Some(rebx) = rebx_extras_mut(&mut sim) {
    rebx_set_param_double(rebx, rebx_ap::force(gr), "c", 10065.32);
}

reb_simulation_integrate(&mut sim, 1000.0);
```

### How the C API maps to Rust

The C uses pointers where safe Rust cannot; three mechanical substitutions are
used consistently (see `src/types.rs` for the full explanation):

| C | Rust |
|---|---|
| `struct rebx_node*` linked list | `Vec`, index 0 = head (the C *prepends*, so order is preserved) |
| `void* value` + type tag | one `enum rebx_param_value` — a wrong-type read is impossible |
| `&sim->particles[i].ap` | `rebx_ap::particle(i)` |
| `&force->ap` | `rebx_ap::force(idx)` |
| `sim->extras` | `sim.extras: Option<Box<dyn Any>>`, downcast by this crate |

So

```c
rebx_set_param_double(rebx, &sim->particles[0].ap, "k2", 0.07);
```

becomes

```rust
rebx_set_param_double(rebx, rebx_ap::particle(0), "k2", 0.07);
```

## Citing

If you use this for published science, cite **Tamayo, Rein, Shi & Hernandez
2019**, MNRAS 491, 2885
([arXiv:1908.05634](https://arxiv.org/abs/1908.05634)), plus the paper(s) for
each effect you enable and the REBOUND papers for the integrator you use. The
per-effect citation table is in [`../rebound_rust/README.md`](../rebound_rust/README.md)
and upstream at <https://reboundx.readthedocs.io>.

**Please do not file issues or pull requests about this port with the upstream
REBOUND or REBOUNDx projects** — they did not write it. See the AI-policy note
in `../rebound_rust/README.md`.

## License

GPL-3.0-or-later, the same license as REBOUNDx. See `LICENSE`.
