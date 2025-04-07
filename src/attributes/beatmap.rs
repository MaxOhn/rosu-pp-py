use pyo3::{
    exceptions::PyTypeError,
    pyclass, pymethods,
    types::{PyAnyMethods, PyDict},
    Bound, Py, PyAny, PyRef, PyResult, Python,
};
use rosu_pp::model::beatmap::{BeatmapAttributes, BeatmapAttributesBuilder, HitWindows};

use crate::{beatmap::PyBeatmap, error::ArgsError, mode::PyGameMode, mods::PyGameMods};

#[pyclass(name = "BeatmapAttributesBuilder")]
#[derive(Default)]
pub struct PyBeatmapAttributesBuilder {
    mode: Option<PyGameMode>,
    is_convert: bool,
    mods: Option<Py<PyAny>>,
    clock_rate: Option<f64>,
    ar: Option<f32>,
    ar_with_mods: bool,
    cs: Option<f32>,
    cs_with_mods: bool,
    hp: Option<f32>,
    hp_with_mods: bool,
    od: Option<f32>,
    od_with_mods: bool,
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

        for (key, value) in kwargs {
            match key.extract()? {
                "map" => {
                    let map = value
                        .extract::<PyRef<'_, PyBeatmap>>()
                        .map_err(|_| PyTypeError::new_err("kwarg 'map': must be a Beatmap"))?;

                    this.set_map(map);
                }
                "mode" => {
                    this.mode =
                        Some(value.extract().map_err(|_| {
                            PyTypeError::new_err("kwarg 'mode': must be a GameMode")
                        })?)
                }
                "is_convert" => {
                    this.is_convert = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'is_convert': must be a bool"))?
                }
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
                kwarg => {
                    let err = format!(
                        "unexpected kwarg '{kwarg}': expected 'map', 'mode', \n\
                        'is_convert', 'mods', 'clock_rate', 'ar', 'ar_with_mods', \n\
                        'cs', 'cs_with_mods', 'hp', 'hp_with_mods', 'od', \n\
                        or 'od_with_mods'"
                    );

                    return Err(ArgsError::new_err(err));
                }
            }
        }

        Ok(this)
    }

    fn build(&self, py: Python<'_>) -> PyResult<PyBeatmapAttributes> {
        let mut builder = BeatmapAttributesBuilder::new();

        builder =
            match PyGameMods::extract(self.mods.as_ref(), self.mode.unwrap_or_default().into(), py)
            {
                Ok(PyGameMods::Lazer(ref mods)) => builder.mods(mods.clone()),
                Ok(PyGameMods::Intermode(ref mods)) => builder.mods(mods),
                Ok(PyGameMods::Legacy(mods)) => builder.mods(mods),
                Err(err) => return Err(err),
            };

        if let Some(mode) = self.mode {
            builder = builder.mode(mode.into(), self.is_convert);
        }

        if let Some(clock_rate) = self.clock_rate {
            builder = builder.clock_rate(clock_rate);
        }

        if let Some(ar) = self.ar {
            builder = builder.ar(ar, self.ar_with_mods);
        }

        if let Some(cs) = self.cs {
            builder = builder.cs(cs, self.cs_with_mods);
        }

        if let Some(hp) = self.hp {
            builder = builder.hp(hp, self.hp_with_mods);
        }

        if let Some(od) = self.od {
            builder = builder.od(od, self.od_with_mods);
        }

        Ok(builder.build().into())
    }

    fn set_map(&mut self, map: PyRef<'_, PyBeatmap>) {
        let map = &map.inner;

        self.mode = Some(map.mode.into());
        self.ar = Some(map.ar);
        self.cs = Some(map.cs);
        self.hp = Some(map.hp);
        self.od = Some(map.od);
        self.is_convert = map.is_convert;
    }

    #[pyo3(signature = (mode, is_convert))]
    fn set_mode(&mut self, mode: Option<PyGameMode>, is_convert: bool) {
        self.mode = mode;
        self.is_convert = is_convert;
    }

    #[pyo3(signature = (mods=None))]
    fn set_mods(&mut self, mods: Option<Py<PyAny>>) {
        self.mods = mods;
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
}

define_class! {
    #[pyclass(name = "BeatmapAttributes", frozen)]
    #[derive(Clone)]
    pub struct PyBeatmapAttributes {
        pub ar: f64!,
        pub od: f64!,
        pub cs: f64!,
        pub hp: f64!,
        pub clock_rate: f64!,
        pub ar_hit_window: f64!,
        pub od_great_hit_window: f64!,
        pub od_ok_hit_window: f64?,
        pub od_meh_hit_window: f64?,
    }
}

impl From<BeatmapAttributes> for PyBeatmapAttributes {
    fn from(attrs: BeatmapAttributes) -> Self {
        let BeatmapAttributes {
            ar,
            od,
            cs,
            hp,
            clock_rate,
            hit_windows:
                HitWindows {
                    ar: ar_hit_window,
                    od_great: od_great_hit_window,
                    od_ok: od_ok_hit_window,
                    od_meh: od_meh_hit_window,
                },
        } = attrs;

        Self {
            ar,
            od,
            cs,
            hp,
            clock_rate,
            ar_hit_window,
            od_great_hit_window,
            od_ok_hit_window,
            od_meh_hit_window,
        }
    }
}
