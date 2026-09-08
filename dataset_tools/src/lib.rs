//! Library facade of the `solveit_db` sidecar, so the integration
//! tests (and any future tool) can reach the pool, schema and server
//! without going through the CLI. The binary in `main.rs` is a thin
//! dispatcher over these modules.

#![deny(warnings)]

pub mod db;
pub mod json;
pub mod serve;
