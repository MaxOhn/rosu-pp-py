use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use pyo3::{
    exceptions::PyTypeError,
    pyclass,
    types::{PyAnyMethods, PyDict},
    Bound, PyResult,
};
use rosu_pp::any::ScoreState;

use crate::error::ArgsError;

#[pyclass(name = "ScoreState")]
#[derive(Clone, Default)]
pub struct PyScoreState {
    #[pyo3(get, set)]
    max_combo: u32,
    #[pyo3(get, set)]
    n_geki: u32,
    #[pyo3(get, set)]
    n_katu: u32,
    #[pyo3(get, set)]
    n300: u32,
    #[pyo3(get, set)]
    n100: u32,
    #[pyo3(get, set)]
    n50: u32,
    #[pyo3(get, set)]
    misses: u32,
}

#[pyo3::pymethods]
impl PyScoreState {
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn new(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        let mut this = Self::default();

        let Some(kwargs) = kwargs else {
            return Ok(this);
        };

        for (key, value) in kwargs {
            match key.extract()? {
                "max_combo" => {
                    this.max_combo = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'max_combo': must be an int"))?
                }
                "n_geki" => {
                    this.n_geki = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n_geki': must be an int"))?
                }
                "n_katu" => {
                    this.n_katu = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n_katu': must be an int"))?
                }
                "n300" => {
                    this.n300 = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n300': must be an int"))?
                }
                "n100" => {
                    this.n100 = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n100': must be an int"))?
                }
                "n50" => {
                    this.n50 = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n50': must be an int"))?
                }
                "misses" => {
                    this.misses = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'misses': must be an int"))?
                }
                kwarg => {
                    let err = format!(
                        "unexpected kwarg '{kwarg}': expected 'max_combo', \n\
                            'n_geki', 'n_katu', 'n300', 'n100', 'n50' or 'misses'",
                    );

                    return Err(ArgsError::new_err(err));
                }
            }
        }

        Ok(this)
    }

    fn __repr__(&self) -> String {
        self.to_string()
    }
}

impl Debug for PyScoreState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self {
            max_combo,
            n_geki,
            n_katu,
            n300,
            n100,
            n50,
            misses,
        } = self;

        f.debug_struct("ScoreState")
            .field("max_combo", max_combo)
            .field("n_geki", n_geki)
            .field("n_katu", n_katu)
            .field("n300", n300)
            .field("n100", n100)
            .field("n50", n50)
            .field("misses", misses)
            .finish()
    }
}

impl Display for PyScoreState {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Debug::fmt(self, f)
    }
}

impl From<&PyScoreState> for ScoreState {
    fn from(state: &PyScoreState) -> Self {
        Self {
            max_combo: state.max_combo,
            n_geki: state.n_geki,
            n_katu: state.n_katu,
            n300: state.n300,
            n100: state.n100,
            n50: state.n50,
            misses: state.misses,
        }
    }
}

impl From<ScoreState> for PyScoreState {
    fn from(state: ScoreState) -> Self {
        Self {
            max_combo: state.max_combo,
            n_geki: state.n_geki,
            n_katu: state.n_katu,
            n300: state.n300,
            n100: state.n100,
            n50: state.n50,
            misses: state.misses,
        }
    }
}
