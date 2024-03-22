use pyo3::{pyclass, PyErr};
use rosu_pp::{
    any::DifficultyAttributes, catch::CatchDifficultyAttributes, mania::ManiaDifficultyAttributes,
    osu::OsuDifficultyAttributes, taiko::TaikoDifficultyAttributes,
};

use crate::{error::ParseError, mode::PyGameMode};

define_class! {
    #[pyclass(name = "DifficultyAttributes", frozen)]
    #[derive(Clone, Default)]
    pub struct PyDifficultyAttributes {
        pub mode: PyGameMode!,
        pub stars: f64!,
        pub is_convert: bool!,
        pub aim: f64?,
        pub speed: f64?,
        pub flashlight: f64?,
        pub slider_factor: f64?,
        pub speed_note_count: f64?,
        pub od: f64?,
        pub hp: f64?,
        pub n_circles: u32?,
        pub n_sliders: u32?,
        pub n_spinners: u32?,
        pub stamina: f64?,
        pub rhythm: f64?,
        pub color: f64?,
        pub peak: f64?,
        pub n_fruits: u32?,
        pub n_droplets: u32?,
        pub n_tiny_droplets: u32?,
        pub n_objects: u32?,
        pub ar: f64?,
        pub hit_window: f64?,
        pub max_combo: u32!,
    }
}

impl From<OsuDifficultyAttributes> for PyDifficultyAttributes {
    fn from(attrs: OsuDifficultyAttributes) -> Self {
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

        Self {
            mode: PyGameMode::Osu,
            stars,
            is_convert: false,
            aim: Some(aim),
            speed: Some(speed),
            flashlight: Some(flashlight),
            slider_factor: Some(slider_factor),
            speed_note_count: Some(speed_note_count),
            ar: Some(ar),
            od: Some(od),
            hp: Some(hp),
            n_circles: Some(n_circles),
            n_sliders: Some(n_sliders),
            n_spinners: Some(n_spinners),
            max_combo,
            ..Self::default()
        }
    }
}

impl From<TaikoDifficultyAttributes> for PyDifficultyAttributes {
    fn from(attrs: TaikoDifficultyAttributes) -> Self {
        let TaikoDifficultyAttributes {
            stamina,
            rhythm,
            color,
            peak,
            hit_window,
            stars,
            max_combo,
            is_convert,
        } = attrs;

        Self {
            mode: PyGameMode::Taiko,
            stars,
            is_convert,
            stamina: Some(stamina),
            rhythm: Some(rhythm),
            color: Some(color),
            peak: Some(peak),
            hit_window: Some(hit_window),
            max_combo,
            ..Self::default()
        }
    }
}

impl From<CatchDifficultyAttributes> for PyDifficultyAttributes {
    fn from(attrs: CatchDifficultyAttributes) -> Self {
        let max_combo = attrs.max_combo();

        let CatchDifficultyAttributes {
            stars,
            ar,
            n_fruits,
            n_droplets,
            n_tiny_droplets,
            is_convert,
        } = attrs;

        Self {
            mode: PyGameMode::Catch,
            stars,
            is_convert,
            ar: Some(ar),
            n_fruits: Some(n_fruits),
            n_droplets: Some(n_droplets),
            n_tiny_droplets: Some(n_tiny_droplets),
            max_combo,
            ..Self::default()
        }
    }
}

impl From<ManiaDifficultyAttributes> for PyDifficultyAttributes {
    fn from(attrs: ManiaDifficultyAttributes) -> Self {
        let ManiaDifficultyAttributes {
            stars,
            hit_window,
            n_objects,
            max_combo,
            is_convert,
        } = attrs;

        Self {
            mode: PyGameMode::Mania,
            stars,
            is_convert,
            hit_window: Some(hit_window),
            n_objects: Some(n_objects),
            max_combo,
            ..Self::default()
        }
    }
}

impl From<DifficultyAttributes> for PyDifficultyAttributes {
    fn from(attrs: DifficultyAttributes) -> Self {
        match attrs {
            DifficultyAttributes::Osu(attrs) => attrs.into(),
            DifficultyAttributes::Taiko(attrs) => attrs.into(),
            DifficultyAttributes::Catch(attrs) => attrs.into(),
            DifficultyAttributes::Mania(attrs) => attrs.into(),
        }
    }
}

impl TryFrom<PyDifficultyAttributes> for DifficultyAttributes {
    type Error = PyErr;

    fn try_from(attrs: PyDifficultyAttributes) -> Result<Self, Self::Error> {
        let PyDifficultyAttributes {
            mode,
            stars,
            is_convert,
            aim,
            speed,
            flashlight,
            slider_factor,
            speed_note_count,
            od,
            hp,
            n_circles,
            n_sliders,
            n_spinners,
            stamina,
            rhythm,
            color,
            peak,
            n_fruits,
            n_droplets,
            n_tiny_droplets,
            n_objects,
            ar,
            hit_window,
            max_combo,
        } = attrs;

        match mode {
            PyGameMode::Osu => {
                if let (
                    Some(aim),
                    Some(speed),
                    Some(flashlight),
                    Some(slider_factor),
                    Some(speed_note_count),
                    Some(ar),
                    Some(od),
                    Some(hp),
                    Some(n_circles),
                    Some(n_sliders),
                    Some(n_spinners),
                ) = (
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
                ) {
                    return Ok(Self::Osu(OsuDifficultyAttributes {
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
                    }));
                }
            }
            PyGameMode::Taiko => {
                if let (Some(stamina), Some(rhythm), Some(color), Some(peak), Some(hit_window)) =
                    (stamina, rhythm, color, peak, hit_window)
                {
                    return Ok(Self::Taiko(TaikoDifficultyAttributes {
                        stamina,
                        rhythm,
                        color,
                        peak,
                        hit_window,
                        stars,
                        max_combo,
                        is_convert,
                    }));
                }
            }
            PyGameMode::Catch => {
                if let (Some(ar), Some(n_fruits), Some(n_droplets), Some(n_tiny_droplets)) =
                    (ar, n_fruits, n_droplets, n_tiny_droplets)
                {
                    return Ok(Self::Catch(CatchDifficultyAttributes {
                        stars,
                        ar,
                        n_fruits,
                        n_droplets,
                        n_tiny_droplets,
                        is_convert,
                    }));
                }
            }
            PyGameMode::Mania => {
                if let (Some(hit_window), Some(n_objects)) = (hit_window, n_objects) {
                    return Ok(Self::Mania(ManiaDifficultyAttributes {
                        stars,
                        hit_window,
                        n_objects,
                        max_combo,
                        is_convert,
                    }));
                }
            }
        }

        Err(ParseError::new_err("invalid difficulty attributes"))
    }
}
