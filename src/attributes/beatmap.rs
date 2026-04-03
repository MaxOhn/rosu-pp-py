use pyo3::{
    exceptions::PyTypeError,
    pyclass, pymethods,
    types::{PyAnyMethods, PyDict},
    Bound, Py, PyAny, PyRef, PyResult, Python,
};
use rosu_pp::model::{
    beatmap::{AdjustedBeatmapAttributes, BeatmapAttributes, BeatmapAttributesBuilder, HitWindows},
    mode::GameMode,
};

use crate::{beatmap::PyBeatmap, error::ArgsError, mode::PyGameMode, mods::PyGameMods};

#[pyclass(name = "BeatmapAttributesBuilder")]
#[derive(Default)]
pub struct PyBeatmapAttributesBuilder {
    inner: BeatmapAttributesBuilder,
    mods: Option<Py<PyAny>>,
    mode: GameMode,
}

#[pymethods]
impl PyBeatmapAttributesBuilder {
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn new(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        let mut this = Self::default();

        let Some(kwargs) = kwargs else {
            return Ok(this);
        };

        let mut mode: Option<PyGameMode> = None;
        let mut is_convert = false;

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
                    "map" => {
                        let map = value
                            .extract::<PyRef<'_, PyBeatmap>>()
                            .map_err(|_| PyTypeError::new_err("kwarg 'map': must be a Beatmap"))?;

                        this.set_map(map);
                    },
                    "mode" => mode = Some(extract!(mode = value as "GameMode")),
                    "is_convert" => is_convert = extract!(is_convert = value as "bool"),
                    "mods" => this.mods = Some(extract!(mods = value as "type that matches GameMods alias")),
                    "clock_rate" => {
                        this.inner.clock_rate(extract!(clock_rate = value as "float"));
                    },
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

        if let Some(mode) = mode {
            this.mode = mode.into();
            this.inner.mode(this.mode, is_convert);
        }

        macro_rules! set_attr {
            ( $attr:ident, $fixed:ident ) => {
                if let Some(value) = $attr {
                    this.inner.$attr(value, $fixed);
                }
            };
        }

        set_attr!(ar, fixed_ar);
        set_attr!(cs, fixed_cs);
        set_attr!(hp, fixed_hp);
        set_attr!(od, fixed_od);

        Ok(this)
    }

    fn build(&mut self, py: Python<'_>) -> PyResult<PyBeatmapAttributes> {
        match PyGameMods::extract(self.mods.as_ref(), self.mode, py) {
            Ok(PyGameMods::Lazer(ref mods)) => self.inner.mods(mods.clone()),
            Ok(PyGameMods::Intermode(ref mods)) => self.inner.mods(mods),
            Ok(PyGameMods::Legacy(mods)) => self.inner.mods(mods),
            Err(err) => return Err(err),
        };

        Ok(self.inner.build().into())
    }

    fn set_map(&mut self, map: PyRef<'_, PyBeatmap>) {
        self.inner.map(&map.inner);
    }

    #[pyo3(signature = (mode, is_convert))]
    fn set_mode(&mut self, mode: PyGameMode, is_convert: bool) {
        self.mode = mode.into();
        self.inner.mode(self.mode, is_convert);
    }

    #[pyo3(signature = (mods=None))]
    fn set_mods(&mut self, mods: Option<Py<PyAny>>) {
        self.mods = mods;
    }

    #[pyo3(signature = (clock_rate))]
    fn set_clock_rate(&mut self, clock_rate: f64) {
        self.inner.clock_rate(clock_rate);
    }

    #[pyo3(signature = (ar, fixed))]
    fn set_ar(&mut self, ar: f32, fixed: bool) {
        self.inner.ar(ar, fixed);
    }

    #[pyo3(signature = (cs, fixed))]
    fn set_cs(&mut self, cs: f32, fixed: bool) {
        self.inner.cs(cs, fixed);
    }

    #[pyo3(signature = (hp, fixed))]
    fn set_hp(&mut self, hp: f32, fixed: bool) {
        self.inner.hp(hp, fixed);
    }

    #[pyo3(signature = (od, fixed))]
    fn set_od(&mut self, od: f32, fixed: bool) {
        self.inner.od(od, fixed);
    }
}

define_class! {
    #[pyclass(name = "BeatmapAttributes", frozen, skip_from_py_object)]
    #[derive(Clone)]
    pub struct PyBeatmapAttributes {
        pub ar: f64!,
        pub base_ar: f32!,
        pub od: f64!,
        pub base_od: f32!,
        pub cs: f32!,
        pub hp: f32!,
        pub clock_rate: f64!,
        pub ar_hit_window: f64?,
        pub od_perfect_hit_window: f64?,
        pub od_great_hit_window: f64?,
        pub od_good_hit_window: f64?,
        pub od_ok_hit_window: f64?,
        pub od_meh_hit_window: f64?,
    }
}

impl From<BeatmapAttributes> for PyBeatmapAttributes {
    fn from(attrs: BeatmapAttributes) -> Self {
        let HitWindows {
            ar: ar_hit_window,
            od_perfect,
            od_great,
            od_good,
            od_ok,
            od_meh,
        } = attrs.hit_windows();

        let AdjustedBeatmapAttributes { ar, cs, hp, od } = attrs.apply_clock_rate();

        Self {
            ar,
            base_ar: attrs.ar(),
            od,
            base_od: attrs.od(),
            cs,
            hp,
            clock_rate: attrs.clock_rate(),
            ar_hit_window,
            od_perfect_hit_window: od_perfect,
            od_great_hit_window: od_great,
            od_good_hit_window: od_good,
            od_ok_hit_window: od_ok,
            od_meh_hit_window: od_meh,
        }
    }
}
