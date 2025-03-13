use error::ConvertError;
use performance::PyHitResultPriority;
use pyo3::prelude::PyModuleMethods;
use pyo3::{pymodule, types::PyModule, Bound, PyResult, Python};

use self::{
    attributes::{
        beatmap::{PyBeatmapAttributes, PyBeatmapAttributesBuilder},
        difficulty::PyDifficultyAttributes,
        performance::PyPerformanceAttributes,
    },
    beatmap::PyBeatmap,
    difficulty::PyDifficulty,
    error::{ArgsError, ParseError},
    gradual::{difficulty::PyGradualDifficulty, performance::PyGradualPerformance},
    mode::PyGameMode,
    performance::PyPerformance,
    score_state::PyScoreState,
    strains::PyStrains,
};

#[macro_use]
mod macros;

mod attributes;
mod beatmap;
mod difficulty;
mod error;
mod gradual;
mod mode;
mod mods;
mod performance;
mod score_state;
mod strains;

#[pymodule]
fn rosu_pp_py(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBeatmap>()?;
    m.add_class::<PyDifficulty>()?;
    m.add_class::<PyPerformance>()?;
    m.add_class::<PyGradualDifficulty>()?;
    m.add_class::<PyGradualPerformance>()?;
    m.add_class::<PyBeatmapAttributesBuilder>()?;

    m.add_class::<PyGameMode>()?;
    m.add_class::<PyScoreState>()?;
    m.add_class::<PyHitResultPriority>()?;

    m.add_class::<PyBeatmapAttributes>()?;
    m.add_class::<PyDifficultyAttributes>()?;
    m.add_class::<PyPerformanceAttributes>()?;
    m.add_class::<PyStrains>()?;

    m.add("ParseError", py.get_type::<ParseError>())?;
    m.add("ArgsError", py.get_type::<ArgsError>())?;
    m.add("ConvertError", py.get_type::<ConvertError>())?;

    Ok(())
}
