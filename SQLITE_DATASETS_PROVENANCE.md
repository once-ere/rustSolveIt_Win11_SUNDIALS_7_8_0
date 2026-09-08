# SQLITE_DATASETS_PROVENANCE.md — the dataset layer, built and verified on this machine

Platform: **Apple Silicon (arm64), macOS** (Darwin 25.6.0). Toolchain:
rustc 1.94.0, cargo 1.94.0, Python 3 (Homebrew), pdflatex (TeX Live),
SQLite 3.46.0 (bundled into the sidecar — the system `sqlite3` CLI is
independent and optional). Every command below was run exactly as
written, from the repository root, on this machine; every number is
captured program output.

This file is self-contained: the complete set of commands to build,
test, verify, execute and display the dataset layer is given here in
full.

---

## 1. What was built

- **Datasets**: every simulation can record itself into a SQLite
  database `datasets/<name>.db` (schema v1: `meta`, `bodies`, `frames`,
  `states`; w-first quaternions; no timestamps). Every one of the 128
  Jupyter notebooks owns one, and SolveIt worked examples 20–38 are
  built around them.
- **The `solveit_db` sidecar** (`dataset_tools/`): the ONE package with
  external crates — `rusqlite` (bundled SQLite amalgamation), `r2d2`,
  `r2d2_sqlite` — exactly as specified for this feature. It owns a
  thread-safe connection pool (size 10) shared across worker threads,
  enables **WAL journal mode** on every connection at creation, and
  sets **`busy_timeout = 5000`**. It is excluded from the simulator
  workspace: `posim` spawns it and speaks line-delimited JSON; the
  simulator still links zero external crates.
- **Language surface** (contextual keywords, command position only):
  `DATASET CREATE/OPEN/CLOSE/RECORD/SNAPSHOT/LIST/INFO/COPY/SAVEAS/`
  `DELETE/PLAY`, `SQL "<any SQLite statement(s)>"` (verbatim
  passthrough — the COMPLETE SQLite grammar), and `MOVIE
  CREATE/DELETE/LIST/PLAY` (movies are 3-D player pages DERIVED from
  datasets). Full reference: `grammar.md` §5.14; complete worked
  session: `grammar.md` §13.
- **Trajectory algebra and curvilinear coordinates** (added in the
  same series of work): `DATASET DIFF <a> <b> AS <out>` and
  `DATASET SUM ...` — frame-aligned component arithmetic with the
  alignment CHECKED (same frame count, bit-equal t grid, same body
  ids; mismatches error naming the frame); `DATASET MAP <src> TO
  <system> [params] AS <out>` — positions through any of the fourteen
  classical curvilinear systems (eleven 3-D + three 2-D, the 2-D ones
  requiring |z| <= 1e-9; output velocities NULL because curvilinear
  components need the metric); and 28 value-level builtins
  (`cart_to_spherical(v)` ... `bipolar_to_cart2(w, a)`), all in pure
  Rust (`special_functions/src/coords.rs`) with round-trip,
  known-value and error-path tests. Reference: `grammar.md` §5.15;
  worked examples 36–38. Measured on this machine: the teacher's
  (A−B)+B−A closure residual `6.938893903907228e-18`; a planar orbit's
  spherical polar angle `1.5707963267948966` with MIN = MAX over 100
  frames; every value-level round trip within `2.2e-16`.

## 2. Build everything

```bash
cd /path/to/rustSolveIt_macos-silicon_SUNDIALS_7_8_0

# the simulator workspace (pure Rust, zero external crates, zero unsafe)
cargo build --workspace --all-targets
cargo build --release --workspace --all-targets

# the SQLite sidecar (first build fetches rusqlite/r2d2 from crates.io;
# dataset_tools/Cargo.lock pins the versions)
cd dataset_tools && cargo build --release && cd ..
```

Measured: both builds finish warning-free. The workspace `Cargo.lock`
still lists only local crates (12 packages) — verify with:

```bash
grep -c "\[\[package\]\]" Cargo.lock        # 12
grep "source = " Cargo.lock | sort -u        # no output: no registry sources
```

## 3. Test the sidecar (WAL + pool + busy timeout, under real contention)

```bash
cd dataset_tools
cargo test
cd ..
```

Measured: **11 passed, 0 failed** (8 unit + 3 integration), including:

- `create_sets_wal_mode_and_busy_timeout` — `PRAGMA journal_mode` comes
  back `wal`, `PRAGMA busy_timeout` comes back `5000`.
- `ten_threads_through_one_pool_lose_nothing` — 10 threads × 200
  simultaneous inserts through ONE cloned pool; all 2000 rows land, no
  `database is locked`.
- `two_pools_on_one_file_interleave_writes_and_reads` — two pools on
  one file (the cross-process picture) with a concurrent reader; all
  600 rows land and the reader's counts never go backwards.
- `serve_protocol_round_trips_append_and_sql` — the real binary driven
  over stdin/stdout exactly as posim drives it, including the WAL
  checkpoint on close.

The multithreaded demonstration, exactly as specified for the feature:

```bash
./dataset_tools/target/release/solveit_db stress /tmp/stress_demo.db 10 500
```

Measured output:

```
stress: 10 threads x 500 rows = 5000 expected, 5000 written, journal_mode=wal
```

## 4. Test the simulator (the new grammar included)

```bash
cargo test --workspace
```

Measured: **638 passed, 0 failed** (622 before the dataset layer, 631
before the coordinate layer; posim 112 → 121, special_functions
233 → 240 with the seven coordinate-map suites). The new posim tests:
`dataset_movie_and_sql_productions_compile`,
`dataset_movie_and_sql_stay_identifiers_off_the_front`,
`dataset_records_run_frames_and_sql_reads_them_back` (end-to-end
through the real sidecar: 10 recorded RUN frames, the analytic
free-fall y(1) = 10 − 9.81/2 read back BY SQL, SAVEAS/DELETE round
trip, a movie generated),
`every_dataset_and_movie_subcommand_is_documented_in_lockstep`
(EBNF ↔ HELP_TEXT ↔ grammar.md ↔ grammar.tex, both directions),
`the_plots_page_is_self_contained` and
`the_player_page_is_self_contained` (no CDN, no fetch, no WebSocket).

## 5. The physics is untouched: the byte-identity gate

The recording hook rides on `integrate::Snapshot` as purely
observational fields — no arithmetic changed anywhere. The gate that
proves it:

```bash
cargo build --release -p posim
bash tools/macos_verify_physics.sh
```

Measured (exit 0):

```
IDENTICAL-MODULO-DOCUMENTED  examples-macos.log vs examples-7.8.0.log (pinned: accepted-divergences-examples.diff)
IDENTICAL  collision-scripts-macos.log == collision-scripts-7.8.0.log
IDENTICAL-MODULO-DOCUMENTED  dynamic-notebooks-macos.log vs dynamic-notebooks-7.8.0.log (pinned: accepted-divergences-dynamic.diff)
```

(The two MODULO lines are the SAME two pre-existing pinned host-libm
divergences documented in `evidence/macos/`; nothing new diverged.)

## 6. Run and verify every example

```bash
# the six self-checking compiled examples
cargo run -p physical_object --release --example kepler_orbit
cargo run -p physical_object --release --example outer_solar_system
cargo run -p physical_object --release --example tumbling_body
cargo run -p physical_object --release --example charged_in_b_field
cargo run -p physical_object --release --example newtons_cradle
cargo run -p physical_object --release --example bouncing_ball_restitution

# the twelve collision scripts
for f in scripts/collisions/*.posim; do
  POSIM_NO_BROWSER=1 ./target/release/posim --script "$f" || echo "FAILED $f"
done

# the THIRTY-EIGHT SolveIt scripts (20-38 are the dataset examples)
for f in scripts/solveit/*.posim; do
  POSIM_NO_BROWSER=1 ./target/release/posim --script "$f" || echo "FAILED $f"
done
```

Measured: all six examples print SUCCESS and exit 0; **all 12 collision
scripts ok**; **all 38 SolveIt scripts ok** (no FAILED line printed).

Highlights measured inside the 16 dataset examples (each number is in
the committed transcripts in `SolveIt.md` §7):

| example | claim | measured by SQL |
|---|---|---|
| 20 | perihelion 1, aphelion 4 | `1.000000001093295`, `4.000000026379791` |
| 21 | free fall matches the closed form | worst error `3.552713678800501e-15` |
| 23 | rebound speeds 10 e | `10.000000000000028 / 7.000000000000018 / 5.000000000000014` |
| 25 | gyroradius 5/9, pitch v_z·T | `0.5555555528977862`, `0.6981317007977318` |
| 26 | \|L\| constant through the flip | `L_min = L_max = 3.0001333303705024` |
| 28 | GGL rod stretch | `7.838174553853605e-14` |
| 31 | Adams vs BDF, by ATTACH + JOIN | worst gap `5.615900633415549e-9` |
| 32 | LAG error = g·dt/2 = 0.24525 | `0.24525000000025088` |
| 35 | quaternions stay unit | `4.440892098500626e-16` |

## 7. The notebooks: 125 executed, eight requirements each

```bash
cargo build --release -p posim
cd dataset_tools && cargo build --release && cd ..
python3 notebooks/_build/regen.py
ls notebooks/*.ipynb | POSIM_NO_BROWSER=1 xargs -P 6 -I{} \
    python3 notebooks/_build/nbrun.py {}
python3 notebooks/_build/nbcheck.py
```

Measured: regen renders **128** notebooks; nbrun ends **128 ok, 0
failed**; nbcheck ends **128/128 notebooks pass all eight
requirements** (R8 — the dataset section and a simulator-independent
`sqlite3` read-back cell — included). The run leaves one dataset per
notebook in `datasets/` and derives the movies in `movies/`.

## 8. Display: the browser artifacts

```bash
# a dataset's plots page (energy, |P| and |L|, per-body positions, top
# view), straight from the notebook language — this opens the browser:
cargo run -p posim --release
#   then type:   dataset play solveit20_kepler
#   (the page is datasets/solveit20_kepler_plots.html)

# a movie (the 3-D player derived from the dataset):
cargo run -p posim --release
#   then type:   movie play solveit20_kepler
#   (the page is movies/solveit20_kepler.html)

# or open the generated pages directly on macOS:
open datasets/solveit20_kepler_plots.html
open movies/solveit20_kepler.html
```

Both pages are self-contained (vanilla JS + canvas, zero external
fetches) and were rendered and inspected in a real browser on this
machine: the movie shows the two spheres, labels, trails, the scrub
bar, speed control and the live t/E/|P|/|L| readout; the plots page
shows the five chart panels drawn from the frames/states tables.

## 9. The other front ends still pass

```bash
python3 recorder/src/record_all.py --check     # legacy recorded videos
python3 tools/gui_smoke.py                     # the 13 live GUI pages
python3 jupyter/test_protocol.py               # the machine-mode wire protocol
POSIM_NO_BROWSER=1 jupyter/.venv/bin/python jupyter/test_kernel.py   # JupyterLab kernel
# (the kernel venv is created once with:
#   uv venv jupyter/.venv
#   uv pip install -p jupyter/.venv/bin/python ipykernel jupyter_client )
```

Measured: **all 13 recordings reproduce byte for byte**; **13 of 13
GUIs pass**; **all protocol checks passed**; **all kernel checks
passed — JupyterLab can drive this kernel**.

## 10. The documents compile

```bash
pdflatex -interaction=nonstopmode grammar.tex
pdflatex -interaction=nonstopmode grammar.tex    # twice: the TOC needs the second pass
pdflatex -interaction=nonstopmode SolveIt.tex
pdflatex -interaction=nonstopmode SolveIt.tex
```

Measured: **grammar.pdf 75 pages**, **SolveIt.pdf 48 pages**, zero
LaTeX errors in either final pass (`grep -c "^!" <log>` = 0).

## 11. The entity index and the encyclopedia

```bash
# the entity index (index_of_entities.html) with the new command families
python3 tools/extract_rust_items.py > index_data/rust_items.jsonl
python3 tools/build_commands.py
python3 tools/build_tierb.py
python3 tools/build_tierc.py
python3 tools/build_catalog.py
python3 tools/build_app.py
python3 tools/verify_index_examples.py
python3 tools/verify_tierb_examples.py

# the notebook encyclopedia (SolveIt_Notebooks_for_rust/, 147 notebooks)
python3 SolveIt_Notebooks_for_rust/_tools/gen.py
# execute every posim-family copy (the mercury and rebound families are
# unchanged and keep their verified runs):
ls SolveIt_Notebooks_for_rust/0[2-6]*/*/*.ipynb | xargs -P 4 -I{} \
    python3 SolveIt_Notebooks_for_rust/_tools/run_copy.py {}
python3 SolveIt_Notebooks_for_rust/_tools/shoot.py
python3 SolveIt_Notebooks_for_rust/_tools/encyclo.py
python3 SolveIt_Notebooks_for_rust/_tools/verify.py
```

Measured, index (final pass): **123 command entries** (the dataset
layer added `cmd.dataset.*` ×11 + `cmd.sql` + `cmd.movie.*` ×4, then
`cmd.dataset.{diff,sum,map}`) plus **28 coordinate-builtin entries**
(`fn.cart_to_spherical` … `fn.bipolar_to_cart2`); verifiers end
**`PASS 1365/1365  (100.0%)`** and **`COMPILED 258/258  (100.0%)`**
(tier-B grew with the public `special_functions::coords` module), and
`index_of_entities.html` embeds 1623/1623 verified examples.

Measured, encyclopedia (final pass): gen.py writes **147** entries
(topic counts 2/38/59/12/13/6/17); the posim-family copies re-execute
ok (final incremental pass: 122/122 ok, 0 FAIL across the four chunk
logs `_tools/exec_posim_0..3.log`; the six rust copies were
byte-identical and needed no re-run); shoot.py captures **124** GUI
images, 0 failed; ENCYCLOPEDIA.md / .html / .pdf rebuilt (12.9 MB PDF,
Appendix B populated); verify.py ends
**`ALL VERIFY GATES PASS (147 notebooks)`**.

## 12. What is committed where

| path | contents |
|---|---|
| `dataset_tools/` | the sidecar: sources, tests, pinned `Cargo.lock`, `README.md` (schema + protocol + architecture) |
| `posim/src/dataset.rs`, `posim/src/movie.rs` | the language side: handle, recording payloads, plots page, movie player |
| `datasets/` | one `.db` per notebook and per dataset example, plus the generated `_plots.html` pages |
| `movies/` | the movies derived from datasets |
| `scripts/solveit/20..38_*.posim` | the sixteen dataset worked examples plus the three trajectory-algebra / coordinate examples |
| `notebooks/` | 128 executed notebooks, each owning a dataset (sections 6, 9, 10) |
| `grammar.md` §5.14–§5.15, §13 / `grammar.tex` / `grammar.pdf` (75 pp) | the full command reference (datasets, SQL, movies, trajectory algebra, the fourteen coordinate systems) and the complete worked session |
| `SolveIt.md` §7 (20–38), §8.3–8.4 / `SolveIt.tex` / `SolveIt.pdf` (48 pp) | the nineteen additional worked examples and the dataset/movie guide |
| `ARCHITECTURE.md` §3.10 | the pinned dataset-subsystem contracts |
| `SolveIt_Notebooks_for_rust/` | the encyclopedia, regenerated at 147 entries with the dataset sections |

## 13. Design decisions recorded

1. **The external-crate boundary.** The standing repository rule is
   zero external dependencies in the simulator workspace; the feature
   specification for datasets explicitly names rusqlite + r2d2. Both
   are honoured at once by the sidecar architecture: `dataset_tools/`
   is EXCLUDED from the workspace (exactly like `sundials_rs`), owns
   its own `Cargo.lock`, and posim reaches SQLite only by spawning
   `solveit_db`. `cargo build --workspace` never touches crates.io.
2. **No timestamps in datasets.** Byte-stable, replayable output is a
   verified repository property; a wall-clock column would break it
   silently. `meta` carries `schema_version`, `title` and `box` only.
3. **`DATASET CREATE` replaces.** Re-running a script or notebook must
   reproduce the same database; COPY and SAVEAS are the preservation
   idioms (both are SQLite `VACUUM INTO`).
4. **`RESET` closes the open dataset.** A dataset describes ONE world.
   The ~40 notebooks whose scripts reset mid-session reopen the file
   afterwards and read back everything recorded up to the reset.
5. **Movies are derived, never recorded.** `MOVIE CREATE` renders
   whatever the dataset holds; the legacy `videos/` recorder pages
   remain and still verify byte-for-byte, but the dataset → movie
   pipeline is the canonical path because the movie is backed by a
   queryable database instead of numbers frozen into HTML.
