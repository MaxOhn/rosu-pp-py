use pyo3::{
    exceptions::PyTypeError,
    pyclass, pymethods,
    types::{PyAnyMethods, PyDict},
    Bound, Py, PyAny, PyResult, Python,
};
use rosu_pp::{model::mode::GameMode, Difficulty};

use crate::{
    attributes::difficulty::PyDifficultyAttributes,
    beatmap::PyBeatmap,
    error::ArgsError,
    gradual::{difficulty::PyGradualDifficulty, performance::PyGradualPerformance},
    mods::PyGameMods,
    performance::PyPerformance,
    strains::PyStrains,
};

#[pyclass(name = "Difficulty")]
#[derive(Default)]
pub struct PyDifficulty {
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
}

#[pymethods]
impl PyDifficulty {
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
                }
            }
        }

        Ok(this)
    }

    fn calculate(&self, map: &PyBeatmap, py: Python<'_>) -> PyResult<PyDifficultyAttributes> {
        Ok(self
            .as_difficulty(map.inner.mode, py)?
            .calculate(&map.inner)
            .into())
    }

    fn strains(&self, map: &PyBeatmap, py: Python<'_>) -> PyResult<PyStrains> {
        Ok(self
            .as_difficulty(map.inner.mode, py)?
            .strains(&map.inner)
            .into())
    }

    fn performance(&self, py: Python<'_>) -> PyPerformance {
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
        } = self;

        PyPerformance {
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
            ..PyPerformance::default()
        }
    }

    fn gradual_difficulty(&self, map: &PyBeatmap, py: Python<'_>) -> PyResult<PyGradualDifficulty> {
        PyGradualDifficulty::new(self, map, py)
    }

    fn gradual_performance(
        &self,
        map: &PyBeatmap,
        py: Python<'_>,
    ) -> PyResult<PyGradualPerformance> {
        PyGradualPerformance::new(self, map, py)
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
}

impl PyDifficulty {
    pub fn as_difficulty(&self, mode: GameMode, py: Python<'_>) -> PyResult<Difficulty> {
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
