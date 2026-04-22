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
    inner: Option<Difficulty>,
    pub(crate) mods: Option<Py<PyAny>>,
}

macro_rules! set {
    ( $this:ident . $field:ident = extract( $( $tt:tt )* ) ) => {
        set!( $this . $field = extract!( $field = $( $tt )* ))
    };
    ( $this:ident . $field:ident = $value:expr ) => {{
        // Expression might propagate out an error so we assign it before
        // passing it to a closure.
        let value = $value;
        $this.set_difficulty(|diff| diff.$field(value));
    }};
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

        let mut ar: Option<f32> = None;
        let mut fixed_ar = false;

        let mut cs: Option<f32> = None;
        let mut fixed_cs = false;

        let mut hp: Option<f32> = None;
        let mut fixed_hp = false;

        let mut od: Option<f32> = None;
        let mut fixed_od = false;

        for (key, value) in kwargs {
            extract_args! {
                match key {
                    "mods" => {
                        this.mods =
                            Some(extract!(mods = value as "type that matches GameMods alias"));
                    },
                    "clock_rate" => set!(this.clock_rate = extract(value as "float")),
                    "passed_objects" => set!(this.passed_objects = extract(value as "int")),
                    "hardrock_offsets" => set!(this.hardrock_offsets = extract(value as "bool")),
                    "lazer" =>  set!(this.lazer = extract(value as "bool")),
                    "ar" => ar = extract!(ar = value as "float"),
                    "fixed_ar" => fixed_ar = extract!(fixed_ar = value as "bool"),
                    "cs" => cs = extract!(cs = value as "float"),
                    "fixed_cs" => fixed_cs = extract!(fixed_cs = value as "bool"),
                    "hp" => hp = extract!(hp = value as "float"),
                    "fixed_hp" => fixed_hp = extract!(fixed_hp = value as "bool"),
                    "od" => od = extract!(od = value as "float"),
                    "fixed_od" => fixed_od = extract!(fixed_od = value as "bool"),
                }
            };
        }

        macro_rules! set_attr {
            ( $attr:ident, $fixed:ident ) => {
                if let Some(value) = $attr {
                    this.set_difficulty(|diff| diff.$attr(value, $fixed));
                }
            };
        }

        set_attr!(ar, fixed_ar);
        set_attr!(cs, fixed_cs);
        set_attr!(hp, fixed_hp);
        set_attr!(od, fixed_od);

        Ok(this)
    }

    fn calculate(&self, map: &PyBeatmap, py: Python<'_>) -> PyResult<PyDifficultyAttributes> {
        Ok(self
            .try_as_difficulty(map.inner.mode, py)?
            .calculate(&map.inner)
            .into())
    }

    fn strains(&self, map: &PyBeatmap, py: Python<'_>) -> PyResult<PyStrains> {
        Ok(self
            .try_as_difficulty(map.inner.mode, py)?
            .strains(&map.inner)
            .into())
    }

    fn performance(&self, py: Python<'_>) -> PyPerformance {
        PyPerformance {
            difficulty: self.clone_py(py),
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
    pub fn set_mods(&mut self, mods: Option<Py<PyAny>>) {
        self.mods = mods;
    }

    #[pyo3(signature = (lazer))]
    pub fn set_lazer(&mut self, lazer: bool) {
        set!(self.lazer = lazer);
    }

    #[pyo3(signature = (clock_rate))]
    pub fn set_clock_rate(&mut self, clock_rate: f64) {
        set!(self.clock_rate = clock_rate);
    }

    #[pyo3(signature = (ar, fixed))]
    pub fn set_ar(&mut self, ar: f32, fixed: bool) {
        self.set_difficulty(|diff| diff.ar(ar, fixed));
    }

    #[pyo3(signature = (cs, fixed))]
    pub fn set_cs(&mut self, cs: f32, fixed: bool) {
        self.set_difficulty(|diff| diff.cs(cs, fixed));
    }

    #[pyo3(signature = (hp, fixed))]
    pub fn set_hp(&mut self, hp: f32, fixed: bool) {
        self.set_difficulty(|diff| diff.hp(hp, fixed));
    }

    #[pyo3(signature = (od, fixed))]
    pub fn set_od(&mut self, od: f32, fixed: bool) {
        self.set_difficulty(|diff| diff.od(od, fixed));
    }

    #[pyo3(signature = (passed_objects))]
    pub fn set_passed_objects(&mut self, passed_objects: u32) {
        set!(self.passed_objects = passed_objects);
    }

    #[pyo3(signature = (hardrock_offsets))]
    pub fn set_hardrock_offsets(&mut self, hardrock_offsets: bool) {
        set!(self.hardrock_offsets = hardrock_offsets);
    }
}

impl PyDifficulty {
    pub(crate) fn clone_py(&self, py: Python<'_>) -> Self {
        Self {
            inner: self.inner.clone(),
            mods: self.mods.as_ref().map(|mods| mods.clone_ref(py)),
        }
    }

    fn set_difficulty(&mut self, set: impl FnOnce(Difficulty) -> Difficulty) {
        self.inner = Some(set(self.inner.take().unwrap_or_default()));
    }

    pub fn try_as_difficulty(&self, mode: GameMode, py: Python<'_>) -> PyResult<Difficulty> {
        let mut difficulty = self.inner.clone().unwrap_or_default();

        difficulty = match PyGameMods::extract(self.mods.as_ref(), mode, py) {
            Ok(PyGameMods::Lazer(ref mods)) => difficulty.mods(mods.clone()),
            Ok(PyGameMods::Intermode(ref mods)) => difficulty.mods(mods),
            Ok(PyGameMods::Legacy(mods)) => difficulty.mods(mods),
            Err(err) => return Err(err),
        };

        Ok(difficulty)
    }
}
