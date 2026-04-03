use pyo3::{
    exceptions::PyTypeError,
    pyclass, pymethods,
    types::{PyAnyMethods, PyDict},
    Bound, Py, PyAny, PyRef, PyResult, Python,
};
use rosu_pp::model::{
    beatmap::{
        AdjustedBeatmapAttributes, BeatmapAttribute, BeatmapAttributes, BeatmapAttributesBuilder,
        HitWindows,
    },
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
        let mut is_convert: Option<bool> = None;

        let mut ar = BeatmapAttribute::None;
        let mut cs = BeatmapAttribute::None;
        let mut hp = BeatmapAttribute::None;
        let mut od = BeatmapAttribute::None;

        let set_attr = |attr: &mut BeatmapAttribute, value| match attr {
            BeatmapAttribute::None | BeatmapAttribute::Value(_) => {
                *attr = BeatmapAttribute::Given(value)
            }
            BeatmapAttribute::Given(old) => *old = value,
            BeatmapAttribute::Fixed(old) => *old = value,
        };

        let fix_attr = |attr: &mut BeatmapAttribute, fixed: bool| {
            if fixed {
                match attr {
                    BeatmapAttribute::None => {
                        *attr = BeatmapAttribute::Fixed(BeatmapAttribute::DEFAULT)
                    }
                    BeatmapAttribute::Value(old) | BeatmapAttribute::Given(old) => {
                        *attr = BeatmapAttribute::Fixed(*old)
                    }
                    BeatmapAttribute::Fixed(_) => {}
                }
            } else {
                match attr {
                    BeatmapAttribute::None => {
                        *attr = BeatmapAttribute::Given(BeatmapAttribute::DEFAULT)
                    }
                    BeatmapAttribute::Value(old) | BeatmapAttribute::Fixed(old) => {
                        *attr = BeatmapAttribute::Given(*old)
                    }
                    BeatmapAttribute::Given(_) => {}
                }
            }
        };

        for (key, value) in kwargs {
            macro_rules! extract {
                ( $kwarg:ident: $ty:literal ) => {
                    value.extract().map_err(|_| {
                        PyTypeError::new_err(concat!(
                            "kwarg '",
                            stringify!($kwarg),
                            "': must be ",
                            stringify!($ty)
                        ))
                    })?
                };
            }

            macro_rules! attr {
                ( set $attr:ident ) => {
                    set_attr(&mut $attr, extract!($attr: "float"))
                };
                ( fix $attr:ident ) => {
                    fix_attr(
                        &mut $attr,
                        value.extract().map_err(|_| PyTypeError::new_err(
                            concat!("kwarg 'fixed_", stringify!($attr), "': must be a bool")
                        ))?,
                    )
                };
            }

            macro_rules! extract_args {
                ( $(
                    $key:literal => $handler:expr,
                )* ) => {
                    match key.extract()? {
                        $( $key => $handler, )*
                        kwarg => {
                            return Err(ArgsError::new_err(extract_args!(
                                @ERR kwarg: $( $key ),*
                            )));
                        }
                    }
                };
                (@ERR $kwarg:ident: $first_field:literal $(, $field:literal )*) => {
                    format!(concat!(
                        "unexpected kwarg '{}': expected ",
                        $first_field,
                        $( ", ", $field, )*
                    ), $kwarg)
                };
            }

            extract_args!(
                "map" => {
                    let map = value
                        .extract::<PyRef<'_, PyBeatmap>>()
                        .map_err(|_| PyTypeError::new_err("kwarg 'map': must be a Beatmap"))?;

                    this.set_map(map);
                },
                "mode" => mode = Some(extract!(mode: "GameMode")),
                "is_convert" => is_convert = Some(extract!(is_convert: "bool")),
                "mods" => this.mods = Some(extract!(mods: "type that matches GameMods alias")),
                "clock_rate" => {
                    this.inner.clock_rate(extract!(clock_rate: "float"));
                },
                "ar" => attr!(set ar),
                "fixed_ar" => attr!(fix ar),
                "cs" => attr!(set cs),
                "fixed_cs" => attr!(fix cs),
                "hp" => attr!(set hp),
                "fixed_hp" => attr!(fix hp),
                "od" => attr!(set od),
                "fixed_od" => attr!(fix od),
            );
        }

        if mode.is_some() || is_convert.is_some() {
            this.mode = mode.unwrap_or_default().into();
            this.inner.mode(this.mode, is_convert.unwrap_or_default());
        }

        macro_rules! apply_attr {
            ( $attr:ident ) => {
                match $attr {
                    BeatmapAttribute::None | BeatmapAttribute::Value(_) => {}
                    BeatmapAttribute::Given(value) => {
                        this.inner.$attr(value, false);
                    }
                    BeatmapAttribute::Fixed(value) => {
                        this.inner.$attr(value, true);
                    }
                }
            };
        }

        apply_attr!(ar);
        apply_attr!(cs);
        apply_attr!(hp);
        apply_attr!(od);

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
