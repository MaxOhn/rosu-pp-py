use std::fmt::{Display, Formatter, Result as FmtResult};

use pyo3::{pyclass, pymethods};
use rosu_pp::{beatmap::BeatmapAttributes, Beatmap, GameMode};

#[pyclass(name = "BeatmapAttributes")]
pub struct PyBeatmapAttributes {
    #[pyo3(get)]
    ar: f64,
    #[pyo3(get)]
    cs: f64,
    #[pyo3(get)]
    hp: f64,
    #[pyo3(get)]
    od: f64,
    #[pyo3(get)]
    ar_hit_window: f64,
    #[pyo3(get)]
    od_hit_window: f64,
    #[pyo3(get)]
    clock_rate: f64,
    #[pyo3(get)]
    bpm: f64,
    #[pyo3(get)]
    mode: u8,
    #[pyo3(get)]
    version: u8,
    #[pyo3(get)]
    n_circles: u32,
    #[pyo3(get)]
    n_sliders: u32,
    #[pyo3(get)]
    n_spinners: u32,
}

impl PyBeatmapAttributes {
    pub fn new(attrs: BeatmapAttributes, mode: GameMode, map: &Beatmap) -> Self {
        Self {
            ar: attrs.ar,
            cs: attrs.cs,
            hp: attrs.hp,
            od: attrs.od,
            ar_hit_window: attrs.hit_windows.ar,
            od_hit_window: attrs.hit_windows.od,
            clock_rate: attrs.clock_rate,
            bpm: map.bpm() * attrs.clock_rate,
            mode: mode as u8,
            version: map.version,
            n_circles: map.n_circles,
            n_sliders: map.n_sliders,
            n_spinners: map.n_spinners,
        }
    }
}

impl Display for PyBeatmapAttributes {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self {
            ar,
            cs,
            hp,
            od,
            ar_hit_window,
            od_hit_window,
            clock_rate,
            bpm,
            mode,
            version,
            n_circles,
            n_sliders,
            n_spinners,
        } = self;

        macro_rules! debug {
            ( $( $field:ident ,)* ) => {
                f.debug_struct("BeatmapAttributes")
                    $( .field(stringify!($field), $field) )*
                    .finish()
            };
        }

        debug! {
            ar,
            cs,
            hp,
            od,
            ar_hit_window,
            od_hit_window,
            clock_rate,
            bpm,
            mode,
            version,
            n_circles,
            n_sliders,
            n_spinners,
        }
    }
}

#[pymethods]
impl PyBeatmapAttributes {
    fn __repr__(&self) -> String {
        self.to_string()
    }
}
