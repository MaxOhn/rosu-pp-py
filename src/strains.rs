use pyo3::pyclass;
use rosu_pp::{
    any::Strains, catch::CatchStrains, mania::ManiaStrains, osu::OsuStrains, taiko::TaikoStrains,
};

use crate::mode::PyGameMode;

type DoubleList = Vec<f64>;

define_class! {
    #[pyclass(name = "Strains", frozen)]
    #[derive(Default)]
    pub struct PyStrains {
        pub mode: PyGameMode!,
        pub section_length: f64!,
        pub aim: DoubleList?,
        pub aim_no_sliders: DoubleList?,
        pub speed: DoubleList?,
        pub flashlight: DoubleList?,
        pub color: DoubleList?,
        pub reading: DoubleList?,
        pub rhythm: DoubleList?,
        pub stamina: DoubleList?,
        pub single_color_stamina: DoubleList?,
        pub movement: DoubleList?,
        pub strains: DoubleList?,
    }
}

impl From<OsuStrains> for PyStrains {
    fn from(strains: OsuStrains) -> Self {
        let OsuStrains {
            aim,
            aim_no_sliders,
            speed,
            flashlight,
        } = strains;

        Self {
            mode: PyGameMode::Osu,
            section_length: OsuStrains::SECTION_LEN,
            aim: Some(aim),
            aim_no_sliders: Some(aim_no_sliders),
            speed: Some(speed),
            flashlight: Some(flashlight),
            ..Self::default()
        }
    }
}

impl From<TaikoStrains> for PyStrains {
    fn from(strains: TaikoStrains) -> Self {
        let TaikoStrains {
            color,
            reading,
            rhythm,
            stamina,
            single_color_stamina,
        } = strains;

        Self {
            mode: PyGameMode::Taiko,
            section_length: TaikoStrains::SECTION_LEN,
            color: Some(color),
            reading: Some(reading),
            rhythm: Some(rhythm),
            stamina: Some(stamina),
            single_color_stamina: Some(single_color_stamina),
            ..Self::default()
        }
    }
}

impl From<CatchStrains> for PyStrains {
    fn from(strains: CatchStrains) -> Self {
        let CatchStrains { movement } = strains;

        Self {
            mode: PyGameMode::Catch,
            section_length: CatchStrains::SECTION_LEN,
            movement: Some(movement),
            ..Self::default()
        }
    }
}

impl From<ManiaStrains> for PyStrains {
    fn from(strains: ManiaStrains) -> Self {
        let ManiaStrains { strains } = strains;

        Self {
            mode: PyGameMode::Mania,
            section_length: ManiaStrains::SECTION_LEN,
            strains: Some(strains),
            ..Self::default()
        }
    }
}

impl From<Strains> for PyStrains {
    fn from(strains: Strains) -> Self {
        match strains {
            Strains::Osu(strains) => strains.into(),
            Strains::Taiko(strains) => strains.into(),
            Strains::Catch(strains) => strains.into(),
            Strains::Mania(strains) => strains.into(),
        }
    }
}
