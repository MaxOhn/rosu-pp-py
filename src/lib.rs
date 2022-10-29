#![deny(clippy::all, nonstandard_style, rust_2018_idioms, unused, warnings)]

use self::{
    beatmap::PyBeatmap,
    calculator::PyCalculator,
    diff_attrs::PyDifficultyAttributes,
    error::{KwargsError, ParseError},
    map_attrs::PyBeatmapAttributes,
    perf_attrs::PyPerformanceAttributes,
};

use pyo3::{pymodule, types::PyModule, PyResult, Python};
use strains::PyStrains;

mod beatmap;
mod calculator;
mod diff_attrs;
mod error;
mod map_attrs;
mod perf_attrs;
mod strains;

#[pymodule]
fn rosu_pp_py(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBeatmap>()?;
    m.add_class::<PyCalculator>()?;

    m.add_class::<PyBeatmapAttributes>()?;
    m.add_class::<PyDifficultyAttributes>()?;
    m.add_class::<PyPerformanceAttributes>()?;
    m.add_class::<PyStrains>()?;

    m.add("ParseError", py.get_type::<ParseError>())?;
    m.add("KwargsError", py.get_type::<KwargsError>())?;

    Ok(())
}
