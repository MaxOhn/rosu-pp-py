use pyo3::{
    exceptions::PyTypeError,
    pyclass, pymethods,
    types::{PyAnyMethods, PyDict},
    Bound, Py, PyAny, PyRef, PyResult, Python,
};
use rosu_pp::{
    any::{DifficultyAttributes, HitResultPriority},
    model::mode::GameMode,
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
    pub(crate) mods: Option<Py<PyAny>>,
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
    pub(crate) lazer: Option<bool>,
    pub(crate) accuracy: Option<f64>,
    pub(crate) combo: Option<u32>,
    pub(crate) large_tick_hits: Option<u32>,
    pub(crate) small_tick_hits: Option<u32>,
    pub(crate) slider_end_hits: Option<u32>,
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
            extract_args! {
                this.key = value {
                    mods: "type that matches GameMods alias",
                    clock_rate: "float",
                    ar: "float",
                    ar_with_mods: "bool",
                    cs: "float",
                    cs_with_mods: "bool",
                    hp: "float",
                    hp_with_mods: "bool",
                    od: "float",
                    od_with_mods: "bool",
                    passed_objects: "int",
                    hardrock_offsets: "bool",
                    lazer: "bool",
                    accuracy: "float",
                    combo: "int",
                    large_tick_hits: "int",
                    small_tick_hits: "int",
                    slider_end_hits: "int",
                    n_geki: "int",
                    n_katu: "int",
                    n300: "int",
                    n100: "int",
                    n50: "int",
                    misses: "int",
                    hitresult_priority: "HitResultPriority",
                }
            }
        }

        Ok(this)
    }

    fn calculate(
        &self,
        args: &Bound<'_, PyAny>,
        py: Python<'_>,
    ) -> PyResult<PyPerformanceAttributes> {
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

        perf = self.apply(perf, py)?;
        let state = perf.generate_state();
        let mut attrs = PyPerformanceAttributes::from(perf.calculate());
        attrs.state = Some(state.into());

        Ok(attrs)
    }

    fn difficulty(&self, py: Python<'_>) -> PyDifficulty {
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
            lazer,
            ..
        } = self;

        PyDifficulty {
            mods: mods.as_ref().map(|mods| mods.clone_ref(py)),
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
            lazer: *lazer,
        }
    }

    #[pyo3(signature = (mods=None))]
    fn set_mods(&mut self, mods: Option<Py<PyAny>>) {
        self.mods = mods;
    }

    #[pyo3(signature = (lazer=None))]
    fn set_lazer(&mut self, lazer: Option<bool>) {
        self.lazer = lazer;
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

    #[pyo3(signature = (n_large_ticks=None))]
    fn set_large_tick_hits(&mut self, n_large_ticks: Option<u32>) {
        self.large_tick_hits = n_large_ticks;
    }

    #[pyo3(signature = (n_small_ticks=None))]
    fn set_small_tick_hits(&mut self, n_small_ticks: Option<u32>) {
        self.small_tick_hits = n_small_ticks;
    }

    #[pyo3(signature = (n_slider_ends=None))]
    fn set_slider_end_hits(&mut self, n_slider_ends: Option<u32>) {
        self.slider_end_hits = n_slider_ends;
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
    fn apply<'a>(&self, mut perf: Performance<'a>, py: Python<'_>) -> PyResult<Performance<'a>> {
        if let Some(accuracy) = self.accuracy {
            perf = perf.accuracy(accuracy);
        }

        if let Some(combo) = self.combo {
            perf = perf.combo(combo);
        }

        if let Some(slider_end_hits) = self.slider_end_hits {
            perf = perf.slider_end_hits(slider_end_hits);
        }

        if let Some(large_tick_hits) = self.large_tick_hits {
            perf = perf.large_tick_hits(large_tick_hits);
        }

        if let Some(small_tick_hits) = self.small_tick_hits {
            perf = perf.small_tick_hits(small_tick_hits);
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

        let mode = match perf {
            Performance::Osu(_) => GameMode::Osu,
            Performance::Taiko(_) => GameMode::Taiko,
            Performance::Catch(_) => GameMode::Catch,
            Performance::Mania(_) => GameMode::Mania,
        };

        let difficulty = self.as_difficulty(mode, py)?;

        Ok(perf
            .hitresult_priority(self.hitresult_priority.into())
            .difficulty(difficulty))
    }

    fn as_difficulty(&self, mode: GameMode, py: Python<'_>) -> PyResult<Difficulty> {
        let mut difficulty = Difficulty::new();

        difficulty = match PyGameMods::extract(self.mods.as_ref(), mode, py) {
            Ok(PyGameMods::Lazer(ref mods)) => difficulty.mods(mods.clone()),
            Ok(PyGameMods::Intermode(ref mods)) => difficulty.mods(mods),
            Ok(PyGameMods::Legacy(mods)) => difficulty.mods(mods),
            Err(err) => return Err(err),
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

        if let Some(lazer) = self.lazer {
            difficulty = difficulty.lazer(lazer);
        }

        Ok(difficulty)
    }
}

#[pyclass(eq, eq_int, name = "HitResultPriority")]
#[derive(Copy, Clone, Default, PartialEq)]
pub enum PyHitResultPriority {
    #[default]
    BestCase,
    WorstCase,
    Fastest,
}

impl From<PyHitResultPriority> for HitResultPriority {
    fn from(priority: PyHitResultPriority) -> Self {
        match priority {
            PyHitResultPriority::BestCase => Self::BestCase,
            PyHitResultPriority::WorstCase => Self::WorstCase,
            PyHitResultPriority::Fastest => Self::Fastest,
        }
    }
}
