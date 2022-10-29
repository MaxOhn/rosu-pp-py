#![deny(clippy::all, nonstandard_style, rust_2018_idioms, unused, warnings)]

use beatmap::PyBeatmap as Beatmap;
use calculator::Calculator;
use pyo3::{pymodule, types::PyModule, PyResult, Python};

mod beatmap;
mod calculator;
mod diff_attrs;
mod error;
mod map_attrs;
mod perf_attrs;
mod strains;

#[pymodule]
fn rosu_pp_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Beatmap>()?;
    m.add_class::<Calculator>()?;

    Ok(())
}
