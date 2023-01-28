use std::borrow::Cow;

use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    pyclass, pymethods,
    types::PyDict,
    PyResult,
};
use rosu_pp::{AnyPP, AnyStars, DifficultyAttributes, GameMode};

use crate::{
    beatmap::PyBeatmap, diff_attrs::PyDifficultyAttributes, error::KwargsError,
    map_attrs::PyBeatmapAttributes, perf_attrs::PyPerformanceAttributes, strains::PyStrains,
};

#[pyclass(name = "Calculator")]
#[derive(Default)]
pub struct PyCalculator {
    attributes: Option<DifficultyAttributes>,
    mode: Option<GameMode>,
    mods: Option<u32>,
    acc: Option<f64>,
    n_geki: Option<usize>,
    n_katu: Option<usize>,
    n300: Option<usize>,
    n100: Option<usize>,
    n50: Option<usize>,
    n_misses: Option<usize>,
    combo: Option<usize>,
    passed_objects: Option<usize>,
    clock_rate: Option<f64>,
}

macro_rules! set_calc {
    ( $calc:ident, $this:ident: $( $field:ident ,)* ) => {
        $(
            if let Some(val) = $this.$field {
                $calc = $calc.$field(val);
            }
        )*
    };
}

#[pymethods]
impl PyCalculator {
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn new(kwargs: Option<&PyDict>) -> PyResult<Self> {
        let kwargs = match kwargs {
            Some(kwargs) => kwargs,
            None => return Ok(Self::default()),
        };

        let mut this = Self::default();

        for (key, value) in kwargs.iter() {
            match key.extract()? {
                "mode" => {
                    let int = value
                        .extract::<u8>()
                        .map_err(|_| PyTypeError::new_err("kwarg 'mode': must be an int"))?;

                    this.mode = match int {
                        0 => Some(GameMode::Osu),
                        1 => Some(GameMode::Taiko),
                        2 => Some(GameMode::Catch),
                        3 => Some(GameMode::Mania),
                        _ => return Err(PyValueError::new_err("invalid mode integer")),
                    }
                }
                "mods" => {
                    this.mods = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'mods': must be an int"))?;
                }
                "n300" => {
                    this.n300 = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n300': must be an int"))?;
                }
                "n100" => {
                    this.n100 = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n100': must be an int"))?;
                }
                "n50" => {
                    this.n50 = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n50': must be an int"))?;
                }
                "n_misses" => {
                    this.n_misses = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n_misses': must be an int"))?;
                }
                "n_geki" => {
                    this.n_geki = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n_geki': must be an int"))?;
                }
                "n_katu" => {
                    this.n_katu = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'n_katu': must be an int"))?;
                }
                "acc" | "accuracy" => {
                    this.acc = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'acc': must be a real number"))?;
                }
                "combo" => {
                    this.combo = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'combo': must be an int"))?;
                }
                "passed_objects" => {
                    this.passed_objects = value.extract().map_err(|_| {
                        PyTypeError::new_err("kwarg 'passed_objects': must be an int")
                    })?;
                }
                "clock_rate" => {
                    this.clock_rate = value.extract().map_err(|_| {
                        PyTypeError::new_err("kwarg 'clock_rate': must be a real number")
                    })?;
                }
                "difficulty" | "attributes" => {
                    let attrs = value.extract::<PyDifficultyAttributes>().map_err(|_| {
                        PyTypeError::new_err("kwarg 'difficulty': must be DifficultyAttributes")
                    })?;

                    this.attributes = Some(attrs.inner);
                }
                kwarg => {
                    let err = format!(
                        "unexpected kwarg '{kwarg}': expected 'mode', 'mods', \n\
                        'n_geki', 'n_katu', 'n300', 'n100', 'n50', 'n_misses', \n\
                        'acc', 'combo', 'passed_objects', 'clock_rate', or 'difficulty'"
                    );

                    return Err(KwargsError::new_err(err));
                }
            }
        }

        Ok(this)
    }

    fn set_mods(&mut self, mods: u32) {
        self.mods = Some(mods);
    }

    fn set_acc(&mut self, acc: f64) {
        self.acc = Some(acc);
    }

    fn set_n_geki(&mut self, n_geki: usize) {
        self.n_geki = Some(n_geki);
    }

    fn set_n_katu(&mut self, n_katu: usize) {
        self.n_katu = Some(n_katu);
    }

    fn set_n300(&mut self, n300: usize) {
        self.n300 = Some(n300);
    }

    fn set_n100(&mut self, n100: usize) {
        self.n100 = Some(n100);
    }

    fn set_n50(&mut self, n50: usize) {
        self.n50 = Some(n50);
    }

    fn set_n_misses(&mut self, n_misses: usize) {
        self.n_misses = Some(n_misses);
    }

    fn set_combo(&mut self, combo: usize) {
        self.combo = Some(combo);
    }

    fn set_passed_objects(&mut self, passed_objects: usize) {
        self.passed_objects = Some(passed_objects);
    }

    fn set_clock_rate(&mut self, clock_rate: f64) {
        self.clock_rate = Some(clock_rate);
    }

    fn set_difficulty(&mut self, difficulty: PyDifficultyAttributes) {
        self.attributes = Some(difficulty.inner);
    }

    fn map_attributes(&self, map: &PyBeatmap) -> PyResult<PyBeatmapAttributes> {
        let (map, mode) = match self.mode {
            Some(mode) => (map.inner.convert_mode(mode), mode),
            None => (Cow::Borrowed(&map.inner), map.inner.mode),
        };

        let mut calc = map.attributes();

        if let Some(mode) = self.mode {
            calc.mode(mode);

            if map.mode != mode && map.mode == GameMode::Osu {
                calc.converted(true);
            }
        }

        if let Some(mods) = self.mods {
            calc.mods(mods);
        }

        if let Some(clock_rate) = self.clock_rate {
            calc.clock_rate(clock_rate);
        }

        Ok(PyBeatmapAttributes::new(calc.build(), mode, map.as_ref()))
    }

    fn difficulty(&self, map: &PyBeatmap) -> PyResult<PyDifficultyAttributes> {
        let mut calc = AnyStars::new(&map.inner);

        set_calc! { calc, self:
            mode,
            mods,
            passed_objects,
            clock_rate,
        };

        Ok(calc.calculate().into())
    }

    fn performance(&self, map: &PyBeatmap) -> PyResult<PyPerformanceAttributes> {
        let mut calc = AnyPP::new(&map.inner);

        set_calc! { calc, self:
            mode,
            mods,
            n_geki,
            n_katu,
            n300,
            n100,
            n50,
            n_misses,
            combo,
            passed_objects,
            clock_rate,
        };

        if let Some(ref attrs) = self.attributes {
            calc = calc.attributes(attrs.to_owned());
        }

        if let Some(acc) = self.acc {
            calc = calc.accuracy(acc);
        }

        Ok(calc.calculate().into())
    }

    fn strains(&self, map: &PyBeatmap) -> PyResult<PyStrains> {
        let mut calc = AnyStars::new(&map.inner);

        set_calc! { calc, self:
            mode,
            mods,
            passed_objects,
            clock_rate,
        };

        Ok(calc.strains().into())
    }
}
