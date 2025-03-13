use pyo3::pyclass;
use rosu_pp::{
    any::PerformanceAttributes, catch::CatchPerformanceAttributes,
    mania::ManiaPerformanceAttributes, osu::OsuPerformanceAttributes,
    taiko::TaikoPerformanceAttributes,
};

use crate::score_state::PyScoreState;

use super::difficulty::PyDifficultyAttributes;

define_class! {
    #[pyclass(name = "PerformanceAttributes", frozen)]
    #[derive(Clone, Default)]
    pub struct PyPerformanceAttributes {
        pub difficulty: PyDifficultyAttributes!,
        pub state: PyScoreState?,
        pub pp: f64!,
        pub pp_aim: f64?,
        pub pp_flashlight: f64?,
        pub pp_speed: f64?,
        pub pp_accuracy: f64?,
        pub effective_miss_count: f64?,
        pub speed_deviation: f64?,
        pub estimated_unstable_rate: f64?,
        pub pp_difficulty: f64?,
    }
}

impl From<OsuPerformanceAttributes> for PyPerformanceAttributes {
    fn from(attrs: OsuPerformanceAttributes) -> Self {
        let OsuPerformanceAttributes {
            difficulty,
            pp,
            pp_acc,
            pp_aim,
            pp_flashlight,
            pp_speed,
            effective_miss_count,
            speed_deviation,
        } = attrs;

        Self {
            difficulty: difficulty.into(),
            pp,
            pp_accuracy: Some(pp_acc),
            pp_aim: Some(pp_aim),
            pp_flashlight: Some(pp_flashlight),
            pp_speed: Some(pp_speed),
            effective_miss_count: Some(effective_miss_count),
            speed_deviation,
            ..Self::default()
        }
    }
}

impl From<TaikoPerformanceAttributes> for PyPerformanceAttributes {
    fn from(attrs: TaikoPerformanceAttributes) -> Self {
        let TaikoPerformanceAttributes {
            difficulty,
            pp,
            pp_acc,
            pp_difficulty,
            effective_miss_count,
            estimated_unstable_rate,
        } = attrs;

        Self {
            difficulty: difficulty.into(),
            pp,
            pp_accuracy: Some(pp_acc),
            pp_difficulty: Some(pp_difficulty),
            effective_miss_count: Some(effective_miss_count),
            estimated_unstable_rate,
            ..Self::default()
        }
    }
}

impl From<CatchPerformanceAttributes> for PyPerformanceAttributes {
    fn from(attrs: CatchPerformanceAttributes) -> Self {
        let CatchPerformanceAttributes { difficulty, pp } = attrs;

        Self {
            difficulty: difficulty.into(),
            pp,
            ..Self::default()
        }
    }
}

impl From<ManiaPerformanceAttributes> for PyPerformanceAttributes {
    fn from(attrs: ManiaPerformanceAttributes) -> Self {
        let ManiaPerformanceAttributes {
            difficulty,
            pp,
            pp_difficulty,
        } = attrs;

        Self {
            difficulty: difficulty.into(),
            pp,
            pp_difficulty: Some(pp_difficulty),
            ..Self::default()
        }
    }
}

impl From<PerformanceAttributes> for PyPerformanceAttributes {
    fn from(attrs: PerformanceAttributes) -> Self {
        match attrs {
            PerformanceAttributes::Osu(attrs) => attrs.into(),
            PerformanceAttributes::Taiko(attrs) => attrs.into(),
            PerformanceAttributes::Catch(attrs) => attrs.into(),
            PerformanceAttributes::Mania(attrs) => attrs.into(),
        }
    }
}
