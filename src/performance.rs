use pyo3::{
    exceptions::PyTypeError,
    pyclass, pymethods,
    types::{PyAnyMethods, PyDict},
    Bound, Py, PyAny, PyRef, PyResult, Python,
};
use rosu_pp::{
    any::{
        hitresult_generator::{Closest, Composable, Fast},
        DifficultyAttributes, HitResultPriority,
    },
    model::mode::GameMode,
    Performance,
};

use crate::{
    attributes::{difficulty::PyDifficultyAttributes, performance::PyPerformanceAttributes},
    beatmap::PyBeatmap,
    difficulty::PyDifficulty,
    error::ArgsError,
    mode::PyGameMode,
};

#[pyclass(name = "Performance")]
#[derive(Default)]
pub struct PyPerformance {
    pub(crate) difficulty: PyDifficulty,
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
    pub(crate) legacy_total_score: Option<u32>,
    pub(crate) hitresult_priority: PyHitResultPriority,
    pub(crate) hitresult_generators: [Option<PyHitResultGenerator>; 4],
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

        let mut ar: Option<f32> = None;
        let mut fixed_ar = false;

        let mut cs: Option<f32> = None;
        let mut fixed_cs = false;

        let mut hp: Option<f32> = None;
        let mut fixed_hp = false;

        let mut od: Option<f32> = None;
        let mut fixed_od = false;

        for (key, value) in kwargs {
            macro_rules! set {
                ( $field:ident: $ty:literal) => {
                    this.$field = extract!($field = value as $ty)
                };
            }

            extract_args! {
                match key {
                    "mods" => this.difficulty.set_mods(Some(extract!(mods = value as "type that matches GameMods alias"))),
                    "clock_rate" => this.difficulty.set_clock_rate(extract!(clock_rate = value as "float")),
                    "passed_objects" => this.difficulty.set_passed_objects(extract!(passed_objects = value as "int")),
                    "hardrock_offsets" => this.difficulty.set_hardrock_offsets(extract!(hardrock_offsets = value as "bool")),
                    "lazer" => this.difficulty.set_lazer(extract!(lazer = value as "bool")),
                    "ar" => ar = extract!(ar = value as "float"),
                    "fixed_ar" => fixed_ar = extract!(fixed_ar = value as "bool"),
                    "cs" => cs = extract!(cs = value as "float"),
                    "fixed_cs" => fixed_cs = extract!(fixed_cs = value as "bool"),
                    "hp" => hp = extract!(hp = value as "float"),
                    "fixed_hp" => fixed_hp = extract!(fixed_hp = value as "bool"),
                    "od" => od = extract!(od = value as "float"),
                    "fixed_od" => fixed_od = extract!(fixed_od = value as "bool"),
                    "accuracy" => set!(accuracy: "float"),
                    "combo" => set!(combo: "int"),
                    "large_tick_hits" => set!(large_tick_hits: "int"),
                    "small_tick_hits" => set!(small_tick_hits: "int"),
                    "slider_end_hits" => set!(slider_end_hits: "int"),
                    "n_geki" => set!(n_geki: "int"),
                    "n_katu" => set!(n_katu: "int"),
                    "n300" => set!(n300: "int"),
                    "n100" => set!(n100: "int"),
                    "n50" => set!(n50: "int"),
                    "misses" => set!(misses: "int"),
                    "legacy_total_score" => set!(legacy_total_score: "int"),
                    "hitresult_priority" => set!(hitresult_priority: "HitResultPriority"),
                }
            }
        }

        macro_rules! set_attr {
            ( $setter:ident ( $attr:ident, $fixed:ident ) ) => {
                if let Some(value) = $attr {
                    this.difficulty.$setter(value, $fixed)
                }
            };
        }

        set_attr!(set_ar(ar, fixed_ar));
        set_attr!(set_cs(cs, fixed_cs));
        set_attr!(set_hp(hp, fixed_hp));
        set_attr!(set_od(od, fixed_od));

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
        self.difficulty.clone_py(py)
    }

    #[pyo3(signature = (mods=None))]
    fn set_mods(&mut self, mods: Option<Py<PyAny>>) {
        self.difficulty.set_mods(mods);
    }

    #[pyo3(signature = (lazer))]
    fn set_lazer(&mut self, lazer: bool) {
        self.difficulty.set_lazer(lazer);
    }

    #[pyo3(signature = (clock_rate))]
    fn set_clock_rate(&mut self, clock_rate: f64) {
        self.difficulty.set_clock_rate(clock_rate);
    }

    #[pyo3(signature = (ar, fixed))]
    fn set_ar(&mut self, ar: f32, fixed: bool) {
        self.difficulty.set_ar(ar, fixed);
    }

    #[pyo3(signature = (cs, fixed))]
    fn set_cs(&mut self, cs: f32, fixed: bool) {
        self.difficulty.set_cs(cs, fixed);
    }

    #[pyo3(signature = (hp, fixed))]
    fn set_hp(&mut self, hp: f32, fixed: bool) {
        self.difficulty.set_hp(hp, fixed);
    }

    #[pyo3(signature = (od, fixed))]
    fn set_od(&mut self, od: f32, fixed: bool) {
        self.difficulty.set_od(od, fixed);
    }

    #[pyo3(signature = (passed_objects))]
    fn set_passed_objects(&mut self, passed_objects: u32) {
        self.difficulty.set_passed_objects(passed_objects);
    }

    #[pyo3(signature = (hardrock_offsets))]
    fn set_hardrock_offsets(&mut self, hardrock_offsets: bool) {
        self.difficulty.set_hardrock_offsets(hardrock_offsets);
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

    #[pyo3(signature = (legacy_total_score=None))]
    fn set_legacy_total_score(&mut self, legacy_total_score: Option<u32>) {
        self.legacy_total_score = legacy_total_score;
    }

    #[pyo3(signature = (hitresult_priority=None))]
    fn set_hitresult_priority(&mut self, hitresult_priority: Option<PyHitResultPriority>) {
        self.hitresult_priority = hitresult_priority.unwrap_or_default();
    }

    #[pyo3(signature = (hitresult_generator, mode=None))]
    fn set_hitresult_generator(
        &mut self,
        hitresult_generator: Option<PyHitResultGenerator>,
        mode: Option<PyGameMode>,
    ) {
        if let Some(mode) = mode {
            self.hitresult_generators[mode as usize] = hitresult_generator;
        } else {
            self.hitresult_generators = [hitresult_generator; 4];
        }
    }
}

impl PyPerformance {
    fn apply<'a>(&self, mut perf: Performance<'a>, py: Python<'_>) -> PyResult<Performance<'a>> {
        let Self {
            difficulty,
            accuracy,
            combo,
            large_tick_hits,
            small_tick_hits,
            slider_end_hits,
            n_geki,
            n_katu,
            n300,
            n100,
            n50,
            misses,
            legacy_total_score,
            hitresult_priority,
            hitresult_generators,
        } = self;

        let mode = match perf {
            Performance::Osu(_) => GameMode::Osu,
            Performance::Taiko(_) => GameMode::Taiko,
            Performance::Catch(_) => GameMode::Catch,
            Performance::Mania(_) => GameMode::Mania,
        };

        perf = perf.difficulty(difficulty.try_as_difficulty(mode, py)?);
        perf = perf.hitresult_priority((*hitresult_priority).into());

        if let Some(accuracy) = accuracy {
            perf = perf.accuracy(*accuracy);
        }

        if let Some(combo) = combo {
            perf = perf.combo(*combo);
        }

        if let Some(slider_end_hits) = slider_end_hits {
            perf = perf.slider_end_hits(*slider_end_hits);
        }

        if let Some(large_tick_hits) = large_tick_hits {
            perf = perf.large_tick_hits(*large_tick_hits);
        }

        if let Some(small_tick_hits) = small_tick_hits {
            perf = perf.small_tick_hits(*small_tick_hits);
        }

        if let Some(n_geki) = n_geki {
            perf = perf.n_geki(*n_geki);
        }

        if let Some(n_katu) = n_katu {
            perf = perf.n_katu(*n_katu);
        }

        if let Some(n300) = n300 {
            perf = perf.n300(*n300);
        }

        if let Some(n100) = n100 {
            perf = perf.n100(*n100);
        }

        if let Some(n50) = n50 {
            perf = perf.n50(*n50);
        }

        if let Some(misses) = misses {
            perf = perf.misses(*misses);
        }

        if let Some(legacy_total_score) = legacy_total_score {
            perf = perf.legacy_total_score(*legacy_total_score);
        }

        // Bridging runtime values to compile-time types
        macro_rules! apply_hitresult_generator {
            // Entry: pass all 4 indices as a "remaining" list
            () => {
                apply_hitresult_generator!(@step [0, 1, 2, 3] [])
            };

            // Still have indices to process
            ( @step [ $i:tt $(, $rest:tt )* ] [ $( $acc:ty ),* ] ) => {
                match hitresult_generators[$i] {
                    None | Some(PyHitResultGenerator::Fast) => {
                        apply_hitresult_generator!(
                            @step [$($rest),*] [$($acc,)* Fast]
                        )
                    }
                    Some(PyHitResultGenerator::Closest) => {
                        apply_hitresult_generator!(
                            @step [$($rest),*] [$($acc,)* Closest]
                        )
                    }
                }
            };

            // No indices left: emit the call
            ( @step [] [$osu:ty, $taiko:ty, $catch:ty, $mania:ty] ) => {
                perf.hitresult_generator::<Composable<$osu, $taiko, $catch, $mania>>()
            };
        }

        perf = apply_hitresult_generator!();

        Ok(perf)
    }
}

#[pyclass(eq, eq_int, name = "HitResultPriority", from_py_object)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
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

#[pyclass(eq, eq_int, name = "HitResultGenerator", from_py_object)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum PyHitResultGenerator {
    #[default]
    Fast,
    Closest,
}
