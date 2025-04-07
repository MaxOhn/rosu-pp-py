use pyo3::{pyclass, pymethods, PyResult, Python};
use rosu_pp::GradualPerformance;

use crate::{
    attributes::performance::PyPerformanceAttributes, beatmap::PyBeatmap, difficulty::PyDifficulty,
    score_state::PyScoreState,
};

#[pyclass(name = "GradualPerformance")]
pub struct PyGradualPerformance {
    inner: GradualPerformance,
}

#[pymethods]
impl PyGradualPerformance {
    #[new]
    pub fn new(difficulty: &PyDifficulty, map: &PyBeatmap, py: Python<'_>) -> PyResult<Self> {
        let difficulty = difficulty.as_difficulty(map.inner.mode, py)?;

        Ok(Self {
            inner: GradualPerformance::new(difficulty, &map.inner),
        })
    }

    fn next(&mut self, state: &PyScoreState) -> Option<PyPerformanceAttributes> {
        self.inner.next(state.into()).map(From::from)
    }

    fn nth(&mut self, state: &PyScoreState, n: usize) -> Option<PyPerformanceAttributes> {
        self.inner.nth(state.into(), n).map(From::from)
    }

    #[getter]
    fn n_remaining(&self) -> usize {
        self.inner.len()
    }
}
