use pyo3::{
    exceptions::PyTypeError,
    pyclass, pymethods,
    types::{PyAnyMethods, PyDict},
    Bound, PyAny, PyRef, PyResult,
};
use rosu_pp::{
    any::{DifficultyAttributes, HitResultPriority},
    Difficulty, Performance,
};

use crate::{
    attributes::{difficulty::PyDifficultyAttributes, performance::PyPerformanceAttributes},
    beatmap::PyBeatmap,
    difficulty::PyDifficulty,
    error::ArgsError,
    mods::PyGameMods,
};

#[pyclass(name = "Performance")]
#[derive(Default)]
pub struct PyPerformance {
    pub(crate) mods: PyGameMods,
    pub(crate) clock_rate: Option<f64>,
    pub(crate) ar: Option<f32>,
    pub(crate) ar_with_mods: bool,
    pub(crate) cs: Option<f32>,
    pub(crate) cs_with_mods: bool,
    pub(crate) hp: Option<f32>,
    pub(crate) hp_with_mods: bool,
    pub(crate) od: Option<f32>,
    pub(crate) od_with_mods: bool,
    pub(crate) passed_objects: Option<u32>,
    pub(crate) hardrock_offsets: Option<bool>,
    pub(crate) accuracy: Option<f64>,
    pub(crate) combo: Option<u32>,
    pub(crate) n_geki: Option<u32>,
    pub(crate) n_katu: Option<u32>,
    pub(crate) n300: Option<u32>,
    pub(crate) n100: Option<u32>,
    pub(crate) n50: Option<u32>,
    pub(crate) misses: Option<u32>,
    pub(crate) hitresult_priority: PyHitResultPriority,
}

#[pymethods]
impl PyPerformance {
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn new(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        let mut this = Self::default();

        let Some(kwargs) = kwargs else {
            return Ok(this);
        };

        for (key, value) in kwargs {
            match key.extract()? {
                "mods" => {
                    this.mods = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'mods': must be GameMods"))?
                }
                "clock_rate" => {
                    this.clock_rate =
                        Some(value.extract().map_err(|_| {
                            PyTypeError::new_err("kwarg 'clock_rate': must be a float")
                        })?)
                }
                "ar" => {
                    this.ar = Some(
                        value
                            .extract()
                            .map_err(|_| PyTypeError::new_err("kwarg 'ar': must be a float"))?,
                    )
                }
                "ar_with_mods" => {
                    this.ar_with_mods = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'ar_with_mods': must be a bool"))?
                }
                "cs" => {
                    this.cs = Some(
                        value
                            .extract()
                            .map_err(|_| PyTypeError::new_err("kwarg 'cs': must be a float"))?,
                    )
                }
                "cs_with_mods" => {
                    this.cs_with_mods = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'cs_with_mods': must be a bool"))?
                }
                "hp" => {
                    this.hp = Some(
                        value
                            .extract()
                            .map_err(|_| PyTypeError::new_err("kwarg 'hp': must be a float"))?,
                    )
                }
                "hp_with_mods" => {
                    this.hp_with_mods = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'hp_with_mods': must be a bool"))?
                }
                "od" => {
                    this.od = Some(
                        value
                            .extract()
                            .map_err(|_| PyTypeError::new_err("kwarg 'od': must be a float"))?,
                    )
                }
                "od_with_mods" => {
                    this.od_with_mods = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'od_with_mods': must be a bool"))?
                }
                "passed_objects" => {
                    this.passed_objects = value.extract().map_err(|_| {
                        PyTypeError::new_err("kwarg 'passed_objects': must be an int")
                    })?
                }
                "hardrock_offsets" => {
                    this.hardrock_offsets = value.extract().map_err(|_| {
                        PyTypeError::new_err("kwarg 'hardrock_offsets': must be a bool")
                    })?
                }
                "accuracy" => {
                    this.accuracy =
                        Some(value.extract().map_err(|_| {
                            PyTypeError::new_err("kwarg 'accuracy': must be a float")
                        })?)
                }
                "combo" => {
                    this.combo = Some(
                        value
                            .extract()
                            .map_err(|_| PyTypeError::new_err("kwarg 'combo': must be an int"))?,
                    )
                }
                "n_geki" => {
                    this.n_geki = Some(
                        value
                            .extract()
                            .map_err(|_| PyTypeError::new_err("kwarg 'n_geki': must be an int"))?,
                    )
                }
                "n_katu" => {
                    this.n_katu = Some(
                        value
                            .extract()
                            .map_err(|_| PyTypeError::new_err("kwarg 'n_katu': must be an int"))?,
                    )
                }
                "n300" => {
                    this.n300 = Some(
                        value
                            .extract()
                            .map_err(|_| PyTypeError::new_err("kwarg 'n300': must be an int"))?,
                    )
                }
                "n100" => {
                    this.n100 = Some(
                        value
                            .extract()
                            .map_err(|_| PyTypeError::new_err("kwarg 'n100': must be an int"))?,
                    )
                }
                "n50" => {
                    this.n50 = Some(
                        value
                            .extract()
                            .map_err(|_| PyTypeError::new_err("kwarg 'n50': must be an int"))?,
                    )
                }
                "misses" => {
                    this.misses = Some(
                        value
                            .extract()
                            .map_err(|_| PyTypeError::new_err("kwarg 'misses': must be an int"))?,
                    )
                }
                "hitresult_priority" => {
                    this.hitresult_priority = value.extract().map_err(|_| {
                        PyTypeError::new_err(
                            "kwarg 'hitresult_priority': must be a HitResultPriority",
                        )
                    })?;
                }
                kwarg => {
                    let err = format!(
                        "unexpected kwarg '{kwarg}': expected 'mods', \n\
                        'clock_rate', 'ar', 'ar_with_mods', 'cs', \n\
                        'cs_with_mods', 'hp', 'hp_with_mods', 'od', \n\
                        'od_with_mods', 'passed_objects', 'hardrock_offsets', \n\
                        'accuracy', 'combo', 'n_geki', 'n_katu', 'n300', 'n100', \n\
                        'n50', 'misses', or 'hitresult_priority'"
                    );

                    return Err(ArgsError::new_err(err));
                }
            }
        }

        Ok(this)
    }

    fn calculate(&self, args: &Bound<'_, PyAny>) -> PyResult<PyPerformanceAttributes> {
        let map;

        let mut perf = if let Ok(attrs) = args.extract::<PyPerformanceAttributes>() {
            Performance::new(DifficultyAttributes::try_from(attrs.difficulty)?)
        } else if let Ok(attrs) = args.extract::<PyDifficultyAttributes>() {
            Performance::new(DifficultyAttributes::try_from(attrs)?)
        } else if let Ok(map_) = args.extract::<PyRef<'_, PyBeatmap>>() {
            map = map_;

            Performance::new(&map.inner)
        } else {
            return Err(ArgsError::new_err(
                "argument must be DifficultyAttributes, PerformanceAttributes, or a Beatmap",
            ));
        };

        perf = self.apply(perf);
        let state = perf.generate_state();
        let mut attrs = PyPerformanceAttributes::from(perf.calculate());
        attrs.state = Some(state.into());

        Ok(attrs)
    }

    fn difficulty(&self) -> PyDifficulty {
        let Self {
            mods,
            clock_rate,
            ar,
            ar_with_mods,
            cs,
            cs_with_mods,
            hp,
            hp_with_mods,
            od,
            od_with_mods,
            passed_objects,
            hardrock_offsets,
            ..
        } = self;

        PyDifficulty {
            mods: mods.clone(),
            clock_rate: *clock_rate,
            ar: *ar,
            ar_with_mods: *ar_with_mods,
            cs: *cs,
            cs_with_mods: *cs_with_mods,
            hp: *hp,
            hp_with_mods: *hp_with_mods,
            od: *od,
            od_with_mods: *od_with_mods,
            passed_objects: *passed_objects,
            hardrock_offsets: *hardrock_offsets,
        }
    }

    #[pyo3(signature = (mods=None))]
    fn set_mods(&mut self, mods: Option<PyGameMods>) {
        self.mods = mods.unwrap_or_default();
    }

    #[pyo3(signature = (clock_rate=None))]
    fn set_clock_rate(&mut self, clock_rate: Option<f64>) {
        self.clock_rate = clock_rate;
    }

    #[pyo3(signature = (ar, ar_with_mods))]
    fn set_ar(&mut self, ar: Option<f32>, ar_with_mods: bool) {
        self.ar = ar;
        self.ar_with_mods = ar_with_mods;
    }

    #[pyo3(signature = (cs, cs_with_mods))]
    fn set_cs(&mut self, cs: Option<f32>, cs_with_mods: bool) {
        self.cs = cs;
        self.cs_with_mods = cs_with_mods;
    }

    #[pyo3(signature = (hp, hp_with_mods))]
    fn set_hp(&mut self, hp: Option<f32>, hp_with_mods: bool) {
        self.hp = hp;
        self.hp_with_mods = hp_with_mods;
    }

    #[pyo3(signature = (od, od_with_mods))]
    fn set_od(&mut self, od: Option<f32>, od_with_mods: bool) {
        self.od = od;
        self.od_with_mods = od_with_mods;
    }

    #[pyo3(signature = (passed_objects=None))]
    fn set_passed_objects(&mut self, passed_objects: Option<u32>) {
        self.passed_objects = passed_objects;
    }

    #[pyo3(signature = (hardrock_offsets=None))]
    fn set_hardrock_offsets(&mut self, hardrock_offsets: Option<bool>) {
        self.hardrock_offsets = hardrock_offsets;
    }

    #[pyo3(signature = (accuracy=None))]
    fn set_accuracy(&mut self, accuracy: Option<f64>) {
        self.accuracy = accuracy;
    }

    #[pyo3(signature = (combo=None))]
    fn set_combo(&mut self, combo: Option<u32>) {
        self.combo = combo;
    }

    #[pyo3(signature = (n_geki=None))]
    fn set_n_geki(&mut self, n_geki: Option<u32>) {
        self.n_geki = n_geki;
    }

    #[pyo3(signature = (n_katu=None))]
    fn set_n_katu(&mut self, n_katu: Option<u32>) {
        self.n_katu = n_katu;
    }

    #[pyo3(signature = (n300=None))]
    fn set_n300(&mut self, n300: Option<u32>) {
        self.n300 = n300;
    }

    #[pyo3(signature = (n100=None))]
    fn set_n100(&mut self, n100: Option<u32>) {
        self.n100 = n100;
    }

    #[pyo3(signature = (n50=None))]
    fn set_n50(&mut self, n50: Option<u32>) {
        self.n50 = n50;
    }

    #[pyo3(signature = (misses=None))]
    fn set_misses(&mut self, misses: Option<u32>) {
        self.misses = misses;
    }

    #[pyo3(signature = (hitresult_priority=None))]
    fn set_hitresult_priority(&mut self, hitresult_priority: Option<PyHitResultPriority>) {
        self.hitresult_priority = hitresult_priority.unwrap_or_default();
    }
}

impl PyPerformance {
    fn apply<'a>(&self, mut perf: Performance<'a>) -> Performance<'a> {
        if let Some(accuracy) = self.accuracy {
            perf = perf.accuracy(accuracy);
        }

        if let Some(combo) = self.combo {
            perf = perf.combo(combo);
        }

        if let Some(n_geki) = self.n_geki {
            perf = perf.n_geki(n_geki);
        }

        if let Some(n_katu) = self.n_katu {
            perf = perf.n_katu(n_katu);
        }

        if let Some(n300) = self.n300 {
            perf = perf.n300(n300);
        }

        if let Some(n100) = self.n100 {
            perf = perf.n100(n100);
        }

        if let Some(n50) = self.n50 {
            perf = perf.n50(n50);
        }

        if let Some(misses) = self.misses {
            perf = perf.misses(misses);
        }

        perf.hitresult_priority(self.hitresult_priority.into())
            .difficulty(self.as_difficulty())
    }

    fn as_difficulty(&self) -> Difficulty {
        let mut difficulty = Difficulty::new();

        difficulty = match self.mods {
            PyGameMods::Lazer(ref mods) => difficulty.mods(mods.clone()),
            PyGameMods::Intermode(ref mods) => difficulty.mods(mods),
            PyGameMods::Legacy(mods) => difficulty.mods(mods),
        };

        if let Some(passed_objects) = self.passed_objects {
            difficulty = difficulty.passed_objects(passed_objects);
        }

        if let Some(clock_rate) = self.clock_rate {
            difficulty = difficulty.clock_rate(clock_rate);
        }

        if let Some(ar) = self.ar {
            difficulty = difficulty.ar(ar, self.ar_with_mods);
        }

        if let Some(cs) = self.cs {
            difficulty = difficulty.cs(cs, self.cs_with_mods);
        }

        if let Some(hp) = self.hp {
            difficulty = difficulty.hp(hp, self.hp_with_mods);
        }

        if let Some(od) = self.od {
            difficulty = difficulty.od(od, self.od_with_mods);
        }

        if let Some(hardrock_offsets) = self.hardrock_offsets {
            difficulty = difficulty.hardrock_offsets(hardrock_offsets);
        }

        difficulty
    }
}

#[pyclass(eq, eq_int, name = "HitResultPriority")]
#[derive(Copy, Clone, Default, PartialEq)]
pub enum PyHitResultPriority {
    #[default]
    BestCase,
    WorstCase,
}

impl From<PyHitResultPriority> for HitResultPriority {
    fn from(priority: PyHitResultPriority) -> Self {
        match priority {
            PyHitResultPriority::BestCase => Self::BestCase,
            PyHitResultPriority::WorstCase => Self::WorstCase,
        }
    }
}
