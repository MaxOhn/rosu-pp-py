use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use pyo3::{
    exceptions::PyTypeError,
    pyclass,
    types::{PyAnyMethods, PyDict},
    Bound, PyResult,
};
use rosu_pp::any::ScoreState;

use crate::error::ArgsError;

#[pyclass(name = "ScoreState", skip_from_py_object)]
#[derive(Clone, Default)]
pub struct PyScoreState {
    #[pyo3(get, set)]
    max_combo: u32,
    #[pyo3(get, set)]
    osu_large_tick_hits: u32,
    #[pyo3(get, set)]
    osu_small_tick_hits: u32,
    #[pyo3(get, set)]
    slider_end_hits: u32,
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
    #[pyo3(get, set)]
    legacy_total_score: Option<u32>,
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
            macro_rules! set {
                ( $field:ident: $ty:literal) => {
                    this.$field = extract!($field = value as $ty)
                };
            }

            extract_args! {
                match key {
                    "max_combo" => set!(max_combo: "int"),
                    "osu_large_tick_hits" => set!(osu_large_tick_hits: "int"),
                    "osu_small_tick_hits" => set!(osu_small_tick_hits: "int"),
                    "slider_end_hits" => set!(slider_end_hits: "int"),
                    "n_geki" => set!(n_geki: "int"),
                    "n_katu" => set!(n_katu: "int"),
                    "n300" => set!(n300: "int"),
                    "n100" => set!(n100: "int"),
                    "n50" => set!(n50: "int"),
                    "misses" => set!(misses: "int"),
                    "legacy_total_score" => set!(legacy_total_score: "int"),
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
            osu_large_tick_hits,
            osu_small_tick_hits,
            slider_end_hits,
            n_geki,
            n_katu,
            n300,
            n100,
            n50,
            misses,
            legacy_total_score,
        } = self;

        f.debug_struct("ScoreState")
            .field("max_combo", max_combo)
            .field("osu_large_tick_hits", osu_large_tick_hits)
            .field("osu_small_tick_hits", osu_small_tick_hits)
            .field("slider_end_hits", slider_end_hits)
            .field("n_geki", n_geki)
            .field("n_katu", n_katu)
            .field("n300", n300)
            .field("n100", n100)
            .field("n50", n50)
            .field("misses", misses)
            .field("legacy_total_score", legacy_total_score)
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
            osu_large_tick_hits: state.osu_large_tick_hits,
            osu_small_tick_hits: state.osu_small_tick_hits,
            slider_end_hits: state.slider_end_hits,
            n_geki: state.n_geki,
            n_katu: state.n_katu,
            n300: state.n300,
            n100: state.n100,
            n50: state.n50,
            misses: state.misses,
            legacy_total_score: state.legacy_total_score,
        }
    }
}

impl From<ScoreState> for PyScoreState {
    fn from(state: ScoreState) -> Self {
        Self {
            max_combo: state.max_combo,
            osu_large_tick_hits: state.osu_large_tick_hits,
            osu_small_tick_hits: state.osu_small_tick_hits,
            slider_end_hits: state.slider_end_hits,
            n_geki: state.n_geki,
            n_katu: state.n_katu,
            n300: state.n300,
            n100: state.n100,
            n50: state.n50,
            misses: state.misses,
            legacy_total_score: state.legacy_total_score,
        }
    }
}
