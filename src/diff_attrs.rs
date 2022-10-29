use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use pyo3::{pyclass, pymethods};
use rosu_pp::{
    catch::CatchDifficultyAttributes, mania::ManiaDifficultyAttributes,
    osu::OsuDifficultyAttributes, taiko::TaikoDifficultyAttributes, DifficultyAttributes,
};

#[pyclass(name = "DifficultyAttributes")]
#[derive(Clone, Debug)]
pub struct PyDifficultyAttributes {
    pub(crate) inner: DifficultyAttributes,
}

impl From<DifficultyAttributes> for PyDifficultyAttributes {
    #[inline]
    fn from(attrs: DifficultyAttributes) -> Self {
        Self { inner: attrs }
    }
}

impl Display for PyDifficultyAttributes {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let borrowed = BorrowedDifficultyAttributes::from(&self.inner);

        Debug::fmt(&borrowed, f)
    }
}

pub(crate) enum BorrowedDifficultyAttributes<'a> {
    Osu(&'a OsuDifficultyAttributes),
    Taiko(&'a TaikoDifficultyAttributes),
    Catch(&'a CatchDifficultyAttributes),
    Mania(&'a ManiaDifficultyAttributes),
}

macro_rules! impl_from {
    ( $( $mode:ident: $attrs:ident, )* ) => {
        $(
            impl<'a> From<&'a $attrs> for BorrowedDifficultyAttributes<'a> {
                #[inline]
                fn from(attrs: &'a $attrs) -> Self {
                    Self::$mode(attrs)
                }
            }
        )*

        impl<'a> From<&'a DifficultyAttributes> for BorrowedDifficultyAttributes<'a> {
            #[inline]
            fn from(attrs: &'a DifficultyAttributes) -> Self {
                match attrs {
                    $( DifficultyAttributes::$mode(attrs) => Self::$mode(attrs), )*
                }
            }
        }
    };
}

impl_from! {
    Osu: OsuDifficultyAttributes,
    Taiko: TaikoDifficultyAttributes,
    Catch: CatchDifficultyAttributes,
    Mania: ManiaDifficultyAttributes,
}

impl Debug for BorrowedDifficultyAttributes<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut debug = f.debug_struct("DifficultyAttributes");

        macro_rules! debug {
            ( $( $field:ident $( , )? )* ) => {
                debug $( .field(stringify!($field), $field) )*;
            }
        }

        match self {
            Self::Osu(attrs) => {
                let OsuDifficultyAttributes {
                    aim,
                    speed,
                    flashlight,
                    slider_factor,
                    speed_note_count,
                    ar,
                    od,
                    hp,
                    n_circles,
                    n_sliders,
                    n_spinners,
                    stars,
                    max_combo,
                } = attrs;

                debug.field("mode", &0_u8);

                debug! {
                    aim,
                    speed,
                    flashlight,
                    slider_factor,
                    speed_note_count,
                    ar,
                    od,
                    hp,
                    n_circles,
                    n_sliders,
                    n_spinners,
                    stars,
                    max_combo,
                }
            }
            Self::Taiko(attrs) => {
                let TaikoDifficultyAttributes {
                    stamina,
                    rhythm,
                    colour,
                    peak,
                    hit_window,
                    stars,
                    max_combo,
                } = attrs;

                debug.field("mode", &1_u8).field("color", colour);
                debug!(stamina, rhythm, peak, hit_window, stars, max_combo);
            }
            Self::Catch(attrs) => {
                let max_combo = attrs.max_combo();

                let CatchDifficultyAttributes {
                    stars,
                    ar,
                    n_fruits,
                    n_droplets,
                    n_tiny_droplets,
                } = attrs;

                debug.field("mode", &2_u8);
                debug!(stars, ar, n_fruits, n_droplets, n_tiny_droplets);
                debug.field("max_combo", &max_combo);
            }
            Self::Mania(attrs) => {
                let ManiaDifficultyAttributes {
                    stars,
                    hit_window,
                    max_combo,
                } = attrs;

                debug.field("mode", &3_u8);
                debug!(stars, hit_window, max_combo);
            }
        }

        debug.finish()
    }
}

macro_rules! getters {
    (
        $(
            $field:ident as $ty:ty: ( $( $mode:ident ),* ),
        )*
    ) => {
        #[pymethods]
        impl PyDifficultyAttributes {
            #[getter]
            fn mode(&self) -> u8 {
                match self.inner {
                    DifficultyAttributes::Osu(_) => 0,
                    DifficultyAttributes::Taiko(_) => 1,
                    DifficultyAttributes::Catch(_) => 2,
                    DifficultyAttributes::Mania(_) => 3,
                }
            }

            #[getter]
            fn max_combo(&self) -> usize {
                match &self.inner {
                    DifficultyAttributes::Osu(attrs) => attrs.max_combo,
                    DifficultyAttributes::Taiko(attrs) => attrs.max_combo,
                    DifficultyAttributes::Catch(attrs) => attrs.max_combo(),
                    DifficultyAttributes::Mania(attrs) => attrs.max_combo,
                }
            }

            #[getter]
            fn color(&self) -> Option<f64> {
                if let DifficultyAttributes::Taiko(ref attrs) = self.inner {
                    Some(attrs.colour)
                } else {
                    None
                }
            }

            fn __repr__(&self) -> String {
                self.to_string()
            }

            $(
                #[getter]
                fn $field(&self) -> Option<$ty> {
                    match &self.inner {
                        $( DifficultyAttributes::$mode(attrs) => Some(attrs.$field), )*
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            )*
        }
    };
}

getters! {
    stars as f64: (Osu, Taiko, Catch, Mania),
    aim as f64: (Osu),
    speed as f64: (Osu),
    flashlight as f64: (Osu),
    slider_factor as f64: (Osu),
    speed_note_count as f64: (Osu),
    od as f64: (Osu),
    n_circles as usize: (Osu),
    n_sliders as usize: (Osu),
    n_spinners as usize: (Osu),
    stamina as f64: (Taiko),
    rhythm as f64: (Taiko),
    peak as f64: (Taiko),
    n_fruits as usize: (Catch),
    n_droplets as usize: (Catch),
    n_tiny_droplets as usize: (Catch),
    ar as f64: (Osu, Catch),
    hit_window as f64: (Taiko, Mania),
}
