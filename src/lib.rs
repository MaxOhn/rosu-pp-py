use std::{
    collections::HashMap,
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult, Write},
    hash::{Hash, Hasher},
};

use pyo3::{
    basic::CompareOp,
    exceptions::{PyException, PyNotImplementedError, PyTypeError},
    prelude::*,
    types::{PyDict, PyIterator, PyTuple},
};
use rosu_pp::{
    beatmap::BeatmapAttributes, catch::CatchPerformanceAttributes,
    mania::ManiaPerformanceAttributes, osu::OsuPerformanceAttributes,
    taiko::TaikoPerformanceAttributes, AnyPP, Beatmap, BeatmapExt, GameMode, PerformanceAttributes,
    Strains as RosuStrains,
};

#[pyclass]
struct Calculator(Beatmap);

#[pymethods]
impl Calculator {
    #[new]
    #[args(kwds = "**")]
    fn new(path: &str, kwds: Option<&PyDict>) -> PyResult<Self> {
        let mut ar = None;
        let mut cs = None;
        let mut hp = None;
        let mut od = None;

        if let Some(dict) = kwds {
            for (key, value) in dict.iter() {
                if let Ok(key) = key.extract() {
                    match key {
                        "ar" => ar = Some(value.extract()?),
                        "cs" => cs = Some(value.extract()?),
                        "hp" => hp = Some(value.extract()?),
                        "od" => od = Some(value.extract()?),
                        _ => {
                            return Err(PyTypeError::new_err(format!(
                                "got an unexpected keyword argument '{}'; \
                                expected 'ar', 'cs', 'hp', 'od'",
                                key,
                            )))
                        }
                    }
                }
            }
        }

        Beatmap::from_path(path)
            .map(|mut map| {
                if let Some(ar) = ar {
                    map.ar = ar;
                }
                if let Some(cs) = cs {
                    map.cs = cs;
                }
                if let Some(hp) = hp {
                    map.hp = hp;
                }
                if let Some(od) = od {
                    map.od = od;
                }

                Self(map)
            })
            .map_err(|e| unwind_error("Failed to parse beatmap", &e))
            .map_err(PyException::new_err)
    }

    fn set_ar(&mut self, ar: f32) {
        self.0.ar = ar;
    }

    fn set_cs(&mut self, cs: f32) {
        self.0.cs = cs;
    }

    fn set_hp(&mut self, hp: f32) {
        self.0.hp = hp;
    }

    fn set_od(&mut self, od: f32) {
        self.0.od = od;
    }

    fn calculate(&mut self, obj: &PyAny) -> PyResult<Vec<CalculateResult>> {
        match obj.extract::<ScoreParams>() {
            Ok(params) => {
                let mods = params.mods;
                let clock_rate = params.clock_rate;

                let calculator = params.apply(AnyPP::new(&self.0));
                let result =
                    CalculateResult::new(calculator.calculate(), &self.0, mods, clock_rate);

                Ok(vec![result])
            }
            Err(_) => {
                let mut mod_diffs = HashMap::new();

                PyIterator::from_object(obj.py(), obj)
                    .map_err(|_| {
                        let py_type = obj.get_type().name().unwrap_or("<unknown type>");

                        format!(
                            "got '{}'; expected 'ScoreParams' or 'Iterable[ScoreParams]'",
                            py_type
                        )
                    })
                    .map_err(PyTypeError::new_err)?
                    .map(|elem| {
                        let params: ScoreParams = elem?.extract()?;
                        let mods = params.mods;
                        let clock_rate = params.clock_rate;
                        let attr_key = params.as_attr_key();

                        let difficulty = mod_diffs
                            .entry(attr_key)
                            .or_insert_with(|| {
                                let mut calculator = self.0.stars().mods(mods);

                                if let Some(passed_objects) = params.passed_objects {
                                    calculator = calculator.passed_objects(passed_objects);
                                }

                                if let Some(clock_rate) = params.clock_rate {
                                    calculator = calculator.clock_rate(clock_rate);
                                }

                                calculator.calculate()
                            })
                            .to_owned();

                        let attrs = params
                            .apply(AnyPP::new(&self.0).attributes(difficulty))
                            .calculate();

                        Ok(CalculateResult::new(attrs, &self.0, mods, clock_rate))
                    })
                    .collect::<Result<Vec<_>, PyErr>>()
            }
        }
    }

    #[args(args = "*")]
    fn strains(&mut self, args: &PyTuple) -> PyResult<Strains> {
        let mods = if let Ok(obj) = args.get_item(0) {
            if let Ok(mods) = obj.extract::<u32>() {
                mods
            } else {
                let py_type = obj.get_type().name().unwrap_or("<unknown type>");
                let err = format!("got '{}'; expected 'int'", py_type);

                return Err(PyTypeError::new_err(err));
            }
        } else {
            0
        };

        Ok(self.0.strains(mods).into())
    }
}

#[pyclass]
#[derive(Clone, Default, PartialEq)]
#[allow(non_snake_case)]
struct Strains {
    #[pyo3(get, set)]
    sectionLength: f64,

    #[pyo3(get, set)]
    color: Option<Vec<f64>>,
    #[pyo3(get, set)]
    rhythm: Option<Vec<f64>>,
    #[pyo3(get, set)]
    staminaLeft: Option<Vec<f64>>,
    #[pyo3(get, set)]
    staminaRight: Option<Vec<f64>>,

    #[pyo3(get, set)]
    aim: Option<Vec<f64>>,
    #[pyo3(get, set)]
    aimNoSliders: Option<Vec<f64>>,
    #[pyo3(get, set)]
    speed: Option<Vec<f64>>,
    #[pyo3(get, set)]
    flashlight: Option<Vec<f64>>,

    #[pyo3(get, set)]
    strains: Option<Vec<f64>>,

    #[pyo3(get, set)]
    movement: Option<Vec<f64>>,
}

#[pymethods]
impl Strains {
    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<bool> {
        match (other.extract::<Self>(), op) {
            (Ok(ref other), CompareOp::Eq) => Ok(self == other),
            (Ok(ref other), CompareOp::Ne) => Ok(self != other),
            _ => Err(PyNotImplementedError::new_err("")),
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

impl From<RosuStrains> for Strains {
    #[inline]
    fn from(strains: RosuStrains) -> Self {
        match strains {
            RosuStrains::Catch(strains) => Self {
                sectionLength: strains.section_len,
                movement: Some(strains.movement),
                ..Default::default()
            },
            RosuStrains::Mania(strains) => Self {
                sectionLength: strains.section_len,
                strains: Some(strains.strains),
                ..Default::default()
            },
            RosuStrains::Osu(strains) => Self {
                sectionLength: strains.section_len,
                aim: Some(strains.aim),
                aimNoSliders: Some(strains.aim_no_sliders),
                speed: Some(strains.speed),
                flashlight: Some(strains.flashlight),
                ..Default::default()
            },
            RosuStrains::Taiko(strains) => Self {
                sectionLength: strains.section_len,
                color: Some(strains.color),
                rhythm: Some(strains.rhythm),
                staminaLeft: Some(strains.stamina_left),
                staminaRight: Some(strains.stamina_right),
                ..Default::default()
            },
        }
    }
}

#[pyclass]
#[derive(Clone, Default, PartialEq)]
struct ScoreParams {
    #[pyo3(get, set)]
    mode: Option<u8>,
    #[pyo3(get, set)]
    mods: u32,
    #[pyo3(get, set)]
    n300: Option<usize>,
    #[pyo3(get, set)]
    n100: Option<usize>,
    #[pyo3(get, set)]
    n50: Option<usize>,
    n_misses: Option<usize>,
    n_katu: Option<usize>,
    #[pyo3(get, set)]
    acc: Option<f64>,
    #[pyo3(get, set)]
    combo: Option<usize>,
    #[pyo3(get, set)]
    score: Option<u32>,
    passed_objects: Option<usize>,
    clock_rate: Option<f64>,
}

#[pyclass]
#[derive(Clone, Default, PartialEq)]
#[allow(non_snake_case)]
struct CalculateResult {
    #[pyo3(get, set)]
    mode: u8,
    #[pyo3(get, set)]
    stars: f64,
    #[pyo3(get, set)]
    pp: f64,
    #[pyo3(get, set)]
    ppAcc: Option<f64>,
    #[pyo3(get, set)]
    ppAim: Option<f64>,
    #[pyo3(get, set)]
    ppFlashlight: Option<f64>,
    #[pyo3(get, set)]
    ppSpeed: Option<f64>,
    #[pyo3(get, set)]
    ppStrain: Option<f64>,

    #[pyo3(get, set)]
    nFruits: Option<usize>,
    #[pyo3(get, set)]
    nDroplets: Option<usize>,
    #[pyo3(get, set)]
    nTinyDroplets: Option<usize>,

    #[pyo3(get, set)]
    aimStrain: Option<f64>,
    #[pyo3(get, set)]
    speedStrain: Option<f64>,
    #[pyo3(get, set)]
    flashlightRating: Option<f64>,
    #[pyo3(get, set)]
    sliderFactor: Option<f64>,

    #[pyo3(get, set)]
    ar: f64,
    #[pyo3(get, set)]
    cs: f64,
    #[pyo3(get, set)]
    hp: f64,
    #[pyo3(get, set)]
    od: f64,
    #[pyo3(get, set)]
    bpm: f64,
    #[pyo3(get, set)]
    clockRate: f64,
    #[pyo3(get, set)]
    nCircles: Option<usize>,
    #[pyo3(get, set)]
    nSliders: Option<usize>,
    #[pyo3(get, set)]
    nSpinners: Option<usize>,
    #[pyo3(get, set)]
    maxCombo: Option<usize>,
}

impl CalculateResult {
    fn new(
        attrs: PerformanceAttributes,
        map: &Beatmap,
        mods: u32,
        clock_rate: Option<f64>,
    ) -> Self {
        let BeatmapAttributes {
            ar,
            cs,
            hp,
            od,
            clock_rate: clock_rate_,
        } = map.attributes().mods(mods);

        let clock_rate = clock_rate.unwrap_or(clock_rate_);
        let bpm = map.bpm() * clock_rate;

        match attrs {
            PerformanceAttributes::Catch(CatchPerformanceAttributes { pp, difficulty }) => Self {
                mode: 2,
                pp,
                stars: difficulty.stars,
                maxCombo: Some(difficulty.n_fruits + difficulty.n_droplets),
                nFruits: Some(difficulty.n_fruits),
                nDroplets: Some(difficulty.n_droplets),
                nTinyDroplets: Some(difficulty.n_tiny_droplets),
                nSpinners: Some(map.n_spinners as usize),
                ar,
                cs,
                hp,
                od,
                bpm,
                clockRate: clock_rate,
                ..Default::default()
            },
            PerformanceAttributes::Mania(ManiaPerformanceAttributes {
                pp,
                pp_acc,
                pp_strain,
                difficulty,
            }) => Self {
                mode: 3,
                pp,
                ppAcc: Some(pp_acc),
                ppStrain: Some(pp_strain),
                stars: difficulty.stars,
                nCircles: Some(map.n_circles as usize),
                nSliders: Some(map.n_sliders as usize),
                ar,
                cs,
                hp,
                od,
                bpm,
                clockRate: clock_rate,
                ..Default::default()
            },
            PerformanceAttributes::Osu(OsuPerformanceAttributes {
                pp,
                pp_acc,
                pp_aim,
                pp_flashlight,
                pp_speed,
                difficulty,
            }) => Self {
                mode: 0,
                pp,
                ppAcc: Some(pp_acc),
                ppAim: Some(pp_aim),
                ppFlashlight: Some(pp_flashlight),
                ppSpeed: Some(pp_speed),
                stars: difficulty.stars,
                maxCombo: Some(difficulty.max_combo),
                aimStrain: Some(difficulty.aim_strain),
                speedStrain: Some(difficulty.speed_strain),
                flashlightRating: Some(difficulty.flashlight_rating),
                sliderFactor: Some(difficulty.slider_factor),
                nCircles: Some(difficulty.n_circles),
                nSliders: Some(difficulty.n_sliders),
                nSpinners: Some(difficulty.n_spinners),
                ar,
                cs,
                hp,
                od,
                bpm,
                clockRate: clock_rate,
                ..Default::default()
            },
            PerformanceAttributes::Taiko(TaikoPerformanceAttributes {
                pp,
                pp_acc,
                pp_strain,
                difficulty,
            }) => Self {
                mode: 1,
                pp,
                ppAcc: Some(pp_acc),
                ppStrain: Some(pp_strain),
                stars: difficulty.stars,
                maxCombo: Some(difficulty.max_combo),
                nCircles: Some(map.n_circles as usize),
                nSliders: Some(map.n_sliders as usize),
                nSpinners: Some(map.n_spinners as usize),
                ar,
                cs,
                hp,
                od,
                bpm,
                clockRate: clock_rate,
                ..Default::default()
            },
        }
    }
}

#[pymethods]
impl CalculateResult {
    #[new]
    fn new_() -> Self {
        Self::default()
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<bool> {
        match (other.extract::<Self>(), op) {
            (Ok(ref other), CompareOp::Eq) => Ok(self == other),
            (Ok(ref other), CompareOp::Ne) => Ok(self != other),
            _ => Err(PyNotImplementedError::new_err("")),
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

fn unwind_error(cause: &str, mut e: &dyn StdError) -> String {
    let mut content = format!("{}: {}\n", cause, e);

    while let Some(src) = e.source() {
        let _ = writeln!(content, "  - caused by: {}", src);
        e = src;
    }

    content
}

impl ScoreParams {
    fn as_attr_key(&self) -> AttributeKey {
        AttributeKey {
            mods: self.mods,
            passed_objects: self.passed_objects,
            clock_rate: self.clock_rate,
        }
    }

    fn apply(self, mut calculator: AnyPP) -> AnyPP {
        let ScoreParams {
            mode,
            mods,
            n300,
            n100,
            n50,
            n_misses,
            n_katu,
            acc,
            combo,
            score,
            passed_objects,
            clock_rate,
        } = self;

        if let Some(mode @ 0..=3) = mode {
            let mode = match mode {
                0 => GameMode::Osu,
                1 => GameMode::Taiko,
                2 => GameMode::Catch,
                3 => GameMode::Mania,
                _ => unreachable!(),
            };

            calculator = calculator.mode(mode);
        }

        if let Some(n300) = n300 {
            calculator = calculator.n300(n300);
        }

        if let Some(n100) = n100 {
            calculator = calculator.n100(n100);
        }

        if let Some(n50) = n50 {
            calculator = calculator.n50(n50);
        }

        if let Some(n_misses) = n_misses {
            calculator = calculator.misses(n_misses);
        }

        if let Some(n_katu) = n_katu {
            calculator = calculator.n_katu(n_katu);
        }

        if let Some(combo) = combo {
            calculator = calculator.combo(combo);
        }

        if let Some(passed_objects) = passed_objects {
            calculator = calculator.passed_objects(passed_objects);
        }

        if let Some(clock_rate) = clock_rate {
            calculator = calculator.clock_rate(clock_rate);
        }

        if let Some(acc) = acc {
            calculator = calculator.accuracy(acc);
        }

        if let Some(score) = score {
            calculator = calculator.score(score);
        }

        calculator.mods(mods)
    }
}

#[pymethods]
impl ScoreParams {
    #[new]
    #[args(kwds = "**")]
    fn new(kwds: Option<&PyDict>) -> PyResult<Self> {
        let mut params = Self::default();

        if let Some(dict) = kwds {
            for (key, value) in dict.iter() {
                if let Ok(key) = key.extract() {
                    match key {
                        "mode" => params.mode = Some(value.extract()?),
                        "mods" => params.mods = value.extract()?,
                        "n300" => params.n300 = value.extract()?,
                        "n100" => params.n100 = value.extract()?,
                        "n50" => params.n50 = value.extract()?,
                        "nMisses" => params.n_misses = value.extract()?,
                        "nKatu" => params.n_katu = value.extract()?,
                        "acc" => params.acc = value.extract()?,
                        "combo" => params.combo = value.extract()?,
                        "score" => params.score = value.extract()?,
                        "passedObjects" => params.passed_objects = value.extract()?,
                        "clockRate" => params.clock_rate = value.extract()?,
                        _ => {
                            return Err(PyTypeError::new_err(format!(
                                "got an unexpected keyword argument '{}'; expected 'mode', 'mods', 'n300', 'n100', \
                                'n50', 'nMisses', 'nKatu', 'acc', 'combo', 'score', 'passedObjects', 'clockRate'",
                                key,
                            )))
                        }
                    }
                }
            }
        }

        Ok(params)
    }

    #[getter(nMisses)]
    fn n_misses(&self) -> Option<usize> {
        self.n_misses
    }

    #[setter(nMisses)]
    fn set_n_misses(&mut self, n_misses: usize) {
        self.n_misses = Some(n_misses);
    }

    #[getter(nKatu)]
    fn n_katu(&self) -> Option<usize> {
        self.n_katu
    }

    #[setter(nKatu)]
    fn set_n_katu(&mut self, n_katu: usize) {
        self.n_katu = Some(n_katu);
    }

    #[getter(passedObjects)]
    fn passed_objects(&self) -> Option<usize> {
        self.passed_objects
    }

    #[setter(passedObjects)]
    fn set_passed_objects(&mut self, passed_objects: usize) {
        self.passed_objects = Some(passed_objects);
    }

    #[getter(clockRate)]
    fn clock_rate(&self) -> Option<f64> {
        self.clock_rate
    }

    #[setter(clockRate)]
    fn set_clock_rate(&mut self, clock_rate: f64) {
        self.clock_rate = Some(clock_rate);
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<bool> {
        match (other.extract::<Self>(), op) {
            (Ok(ref other), CompareOp::Eq) => Ok(self == other),
            (Ok(ref other), CompareOp::Ne) => Ok(self != other),
            _ => Err(PyNotImplementedError::new_err("")),
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

impl Display for Strains {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut s = f.debug_struct("Strains");

        s.field("sectionLength", &self.sectionLength);

        macro_rules! display_field {
            ($self:ident, $s:ident: $($field:ident,)*) => {
                $(
                    if let Some(ref field) = $self.$field {
                        $s.field(stringify!($field), field);
                    }
                )*
            }
        }

        display_field! {
            self, s:
            color,
            rhythm,
            staminaLeft,
            staminaRight,
            aim,
            aimNoSliders,
            speed,
            flashlight,
            strains,
            movement,
        }

        s.finish()
    }
}

impl Display for CalculateResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut s = f.debug_struct("CalculateResult");

        s.field("mode", &self.mode)
            .field("stars", &self.stars)
            .field("pp", &self.pp);

        if let Some(ref pp_acc) = self.ppAcc {
            s.field("ppAcc", pp_acc);
        }

        if let Some(ref pp_aim) = self.ppAim {
            s.field("ppAim", pp_aim);
        }

        if let Some(ref pp_flashlight) = self.ppFlashlight {
            s.field("ppFlashlight", pp_flashlight);
        }

        if let Some(ref pp_speed) = self.ppSpeed {
            s.field("ppSpeed", pp_speed);
        }

        if let Some(ref pp_strain) = self.ppStrain {
            s.field("ppStrain", pp_strain);
        }

        if let Some(ref n_fruits) = self.nFruits {
            s.field("nFruits", n_fruits);
        }

        if let Some(ref n_droplets) = self.nDroplets {
            s.field("nDroplets", n_droplets);
        }

        if let Some(ref n_tiny_droplets) = self.nTinyDroplets {
            s.field("nTinyDroplets", n_tiny_droplets);
        }

        if let Some(ref aim_strain) = self.aimStrain {
            s.field("aimStrain", aim_strain);
        }

        if let Some(ref speed_strain) = self.speedStrain {
            s.field("speedStrain", speed_strain);
        }

        if let Some(ref flashlight_rating) = self.flashlightRating {
            s.field("flashlightRating", flashlight_rating);
        }

        if let Some(ref slider_factor) = self.sliderFactor {
            s.field("sliderFactor", slider_factor);
        }

        s.field("ar", &self.ar)
            .field("cs", &self.cs)
            .field("hp", &self.hp)
            .field("od", &self.od)
            .field("bpm", &self.bpm)
            .field("clockRate", &self.clockRate);

        if let Some(ref n_circles) = self.nCircles {
            s.field("nCircles", n_circles);
        }

        if let Some(ref n_sliders) = self.nSliders {
            s.field("nSliders", n_sliders);
        }

        if let Some(ref n_spinners) = self.nSpinners {
            s.field("nSpinners", n_spinners);
        }

        if let Some(ref combo) = self.maxCombo {
            s.field("maxCombo", combo);
        }

        s.finish()
    }
}

impl Display for ScoreParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "ScoreParams {{ \
            mode: {}, \
            mods: {}, \
            n300: {}, \
            n100: {}, \
            n50: {}, \
            nMisses: {}, \
            nKatu: {}, \
            acc: {}, \
            combo: {}, \
            score: {}, \
            passedObjects: {}, \
            clockRate: {} \
        }}",
            match self.mode {
                Some(ref mode) => mode as &dyn Display,
                None => &"None" as &dyn Display,
            },
            self.mods,
            match self.n300 {
                Some(ref n300) => n300 as &dyn Display,
                None => &"None" as &dyn Display,
            },
            match self.n100 {
                Some(ref n100) => n100 as &dyn Display,
                None => &"None" as &dyn Display,
            },
            match self.n50 {
                Some(ref n50) => n50 as &dyn Display,
                None => &"None" as &dyn Display,
            },
            match self.n_misses {
                Some(ref n_misses) => n_misses as &dyn Display,
                None => &"None" as &dyn Display,
            },
            match self.n_katu {
                Some(ref n_katu) => n_katu as &dyn Display,
                None => &"None" as &dyn Display,
            },
            match self.acc {
                Some(ref acc) => acc as &dyn Display,
                None => &"None" as &dyn Display,
            },
            match self.combo {
                Some(ref combo) => combo as &dyn Display,
                None => &"None" as &dyn Display,
            },
            match self.score {
                Some(ref score) => score as &dyn Display,
                None => &"None" as &dyn Display,
            },
            match self.passed_objects {
                Some(ref passed_objects) => passed_objects as &dyn Display,
                None => &"None" as &dyn Display,
            },
            match self.clock_rate {
                Some(ref clock_rate) => clock_rate as &dyn Display,
                None => &"None" as &dyn Display,
            },
        )
    }
}

#[pymodule]
fn rosu_pp_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ScoreParams>()?;
    m.add_class::<Calculator>()?;
    m.add_class::<CalculateResult>()?;
    m.add_class::<Strains>()?;

    Ok(())
}

struct AttributeKey {
    mods: u32,
    passed_objects: Option<usize>,
    clock_rate: Option<f64>,
}

impl Hash for AttributeKey {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mods.hash(state);
        self.passed_objects.hash(state);
        (&self.clock_rate as *const _ as *const Option<u64>).hash(state);
    }
}

impl PartialEq for AttributeKey {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.mods == other.mods
            && self.passed_objects == other.passed_objects
            && self.clock_rate == other.clock_rate
    }
}

impl Eq for AttributeKey {}
