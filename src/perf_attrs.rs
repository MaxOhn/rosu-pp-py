use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use pyo3::{pyclass, pymethods};
use rosu_pp::{
    catch::CatchPerformanceAttributes, mania::ManiaPerformanceAttributes,
    osu::OsuPerformanceAttributes, taiko::TaikoPerformanceAttributes, PerformanceAttributes,
};

use crate::diff_attrs::{BorrowedDifficultyAttributes, PyDifficultyAttributes};

#[pyclass(name = "PerformanceAttributes")]
#[derive(Debug)]
pub struct PyPerformanceAttributes {
    inner: PerformanceAttributes,
}

impl From<PerformanceAttributes> for PyPerformanceAttributes {
    #[inline]
    fn from(attrs: PerformanceAttributes) -> Self {
        Self { inner: attrs }
    }
}

impl Display for PyPerformanceAttributes {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut debug = f.debug_struct("PerformanceAttributes");

        macro_rules! debug {
            ( $( $field:ident $( , )? )* ) => {
                debug $( .field(stringify!($field), $field) )*;
            }
        }

        match &self.inner {
            PerformanceAttributes::Osu(attrs) => {
                let OsuPerformanceAttributes {
                    difficulty,
                    pp,
                    pp_acc,
                    pp_aim,
                    pp_flashlight,
                    pp_speed,
                    effective_miss_count,
                } = attrs;

                let borrowed = BorrowedDifficultyAttributes::from(difficulty);
                debug.field("mode", &0_u8).field("difficulty", &borrowed);

                debug! {
                    pp,
                    pp_acc,
                    pp_aim,
                    pp_flashlight,
                    pp_speed,
                    effective_miss_count,
                };
            }
            PerformanceAttributes::Taiko(attrs) => {
                let TaikoPerformanceAttributes {
                    difficulty,
                    pp,
                    pp_acc,
                    pp_difficulty,
                    effective_miss_count,
                } = attrs;

                let borrowed = BorrowedDifficultyAttributes::from(difficulty);
                debug.field("mode", &1_u8).field("difficulty", &borrowed);
                debug!(pp, pp_acc, pp_difficulty, effective_miss_count);
            }
            PerformanceAttributes::Catch(attrs) => {
                let CatchPerformanceAttributes { difficulty, pp } = attrs;

                let borrowed = BorrowedDifficultyAttributes::from(difficulty);
                debug.field("mode", &2_u8).field("difficulty", &borrowed);
                debug!(pp);
            }
            PerformanceAttributes::Mania(attrs) => {
                let ManiaPerformanceAttributes {
                    difficulty,
                    pp,
                    pp_difficulty,
                } = attrs;

                let borrowed = BorrowedDifficultyAttributes::from(difficulty);
                debug.field("mode", &3_u8).field("difficulty", &borrowed);
                debug!(pp, pp_difficulty);
            }
        }

        debug.finish()
    }
}

macro_rules! getters {
    (
        $(
            $field:ident: ( $( $mode:ident ),* ),
        )*
    ) => {
        #[pymethods]
        impl PyPerformanceAttributes {
            #[getter]
            fn mode(&self) -> u8 {
                match self.inner {
                    PerformanceAttributes::Osu(_) => 0,
                    PerformanceAttributes::Taiko(_) => 1,
                    PerformanceAttributes::Catch(_) => 2,
                    PerformanceAttributes::Mania(_) => 3,
                }
            }

            #[getter]
            fn difficulty(&self) -> PyDifficultyAttributes {
                 self.inner.difficulty_attributes().into()
            }

            fn __repr__(&self) -> String {
                self.to_string()
            }

            $(
                #[getter]
                fn $field(&self) -> Option<f64> {
                    match &self.inner {
                        $( PerformanceAttributes::$mode(attrs) => Some(attrs.$field), )*
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            )*
        }
    };
}

getters! {
    pp: (Osu, Taiko, Catch, Mania),
    pp_aim: (Osu),
    pp_flashlight: (Osu),
    pp_speed: (Osu),
    pp_acc: (Osu, Taiko),
    effective_miss_count: (Osu, Taiko),
    pp_difficulty: (Taiko, Mania),
}
