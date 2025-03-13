use pyo3::{pyclass, pymethods, PyRef, PyResult, Python};
use rosu_pp::GradualDifficulty;

use crate::{
    attributes::difficulty::PyDifficultyAttributes, beatmap::PyBeatmap, difficulty::PyDifficulty,
};

#[pyclass(name = "GradualDifficulty")]
pub struct PyGradualDifficulty {
    inner: GradualDifficulty,
}

#[pymethods]
impl PyGradualDifficulty {
    #[new]
    pub fn new(difficulty: &PyDifficulty, map: &PyBeatmap, py: Python<'_>) -> PyResult<Self> {
        let difficulty = difficulty.as_difficulty(map.inner.mode, py)?;

        Ok(Self {
            inner: GradualDifficulty::new(difficulty, &map.inner),
        })
    }

    fn next(&mut self) -> Option<PyDifficultyAttributes> {
        self.inner.next().map(From::from)
    }

    fn nth(&mut self, n: usize) -> Option<PyDifficultyAttributes> {
        self.inner.nth(n).map(From::from)
    }

    #[getter]
    fn n_remaining(&self) -> usize {
        self.inner.len()
    }

    fn __iter__(this: PyRef<'_, Self>) -> PyRef<'_, Self> {
        this
    }

    fn __next__(&mut self) -> Option<PyDifficultyAttributes> {
        self.next()
    }
}
