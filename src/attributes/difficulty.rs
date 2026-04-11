use pyo3::{pyclass, PyErr};
use rosu_pp::{
    any::DifficultyAttributes, catch::CatchDifficultyAttributes, mania::ManiaDifficultyAttributes,
    osu::OsuDifficultyAttributes, taiko::TaikoDifficultyAttributes,
};

use crate::{error::ParseError, mode::PyGameMode};

define_class! {
    #[pyclass(name = "DifficultyAttributes", frozen, from_py_object)]
    #[derive(Clone, Default)]
    pub struct PyDifficultyAttributes {
        pub mode: PyGameMode!,
        pub stars: f64!,
        pub is_convert: bool!,
        pub aim: f64?,
        pub aim_difficult_slider_count: f64?,
        pub speed: f64?,
        pub flashlight: f64?,
        pub slider_factor: f64?,
        pub aim_top_weighted_slider_factor: f64?,
        pub speed_top_weighted_slider_factor: f64?,
        pub speed_note_count: f64?,
        pub aim_difficult_strain_count: f64?,
        pub speed_difficult_strain_count: f64?,
        pub nested_score_per_object: f64?,
        pub legacy_score_base_multiplier: f64?,
        pub maximum_legacy_combo_score: f64?,
        pub hp: f64?,
        pub n_circles: u32?,
        pub n_sliders: u32?,
        pub n_large_ticks: u32?,
        pub n_spinners: u32?,
        pub stamina: f64?,
        pub reading: f64?,
        pub rhythm: f64?,
        pub color: f64?,
        pub n_fruits: u32?,
        pub n_droplets: u32?,
        pub n_tiny_droplets: u32?,
        pub n_objects: u32?,
        pub n_hold_notes: u32?,
        pub ar: f64?,
        pub preempt: f64?,
        pub great_hit_window: f64?,
        pub ok_hit_window: f64?,
        pub meh_hit_window: f64?,
        pub mono_stamina_factor: f64?,
        pub mechanical_difficulty: f64?,
        pub consistency_factor: f64?,
        pub max_combo: u32!,
    }
}

impl From<OsuDifficultyAttributes> for PyDifficultyAttributes {
    fn from(attrs: OsuDifficultyAttributes) -> Self {
        let OsuDifficultyAttributes {
            aim,
            aim_difficult_slider_count,
            speed,
            flashlight,
            slider_factor,
            aim_top_weighted_slider_factor,
            speed_top_weighted_slider_factor,
            speed_note_count,
            aim_difficult_strain_count,
            speed_difficult_strain_count,
            nested_score_per_object,
            legacy_score_base_multiplier,
            maximum_legacy_combo_score,
            ar,
            great_hit_window,
            ok_hit_window,
            meh_hit_window,
            hp,
            n_circles,
            n_sliders,
            n_large_ticks,
            n_spinners,
            stars,
            max_combo,
        } = attrs;

        Self {
            mode: PyGameMode::Osu,
            stars,
            is_convert: false,
            aim: Some(aim),
            aim_difficult_slider_count: Some(aim_difficult_slider_count),
            speed: Some(speed),
            flashlight: Some(flashlight),
            slider_factor: Some(slider_factor),
            aim_top_weighted_slider_factor: Some(aim_top_weighted_slider_factor),
            speed_top_weighted_slider_factor: Some(speed_top_weighted_slider_factor),
            speed_note_count: Some(speed_note_count),
            aim_difficult_strain_count: Some(aim_difficult_strain_count),
            speed_difficult_strain_count: Some(speed_difficult_strain_count),
            nested_score_per_object: Some(nested_score_per_object),
            legacy_score_base_multiplier: Some(legacy_score_base_multiplier),
            maximum_legacy_combo_score: Some(maximum_legacy_combo_score),
            ar: Some(ar),
            great_hit_window: Some(great_hit_window),
            ok_hit_window: Some(ok_hit_window),
            meh_hit_window: Some(meh_hit_window),
            hp: Some(hp),
            n_circles: Some(n_circles),
            n_sliders: Some(n_sliders),
            n_large_ticks: Some(n_large_ticks),
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
            reading,
            great_hit_window,
            ok_hit_window,
            mono_stamina_factor,
            mechanical_difficulty,
            consistency_factor,
            stars,
            max_combo,
            is_convert,
        } = attrs;

        Self {
            mode: PyGameMode::Taiko,
            stars,
            is_convert,
            stamina: Some(stamina),
            reading: Some(reading),
            rhythm: Some(rhythm),
            color: Some(color),
            great_hit_window: Some(great_hit_window),
            ok_hit_window: Some(ok_hit_window),
            mono_stamina_factor: Some(mono_stamina_factor),
            mechanical_difficulty: Some(mechanical_difficulty),
            consistency_factor: Some(consistency_factor),
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
            preempt,
            n_fruits,
            n_droplets,
            n_tiny_droplets,
            is_convert,
        } = attrs;

        Self {
            mode: PyGameMode::Catch,
            stars,
            preempt: Some(preempt),
            is_convert,
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
            n_objects,
            n_hold_notes,
            max_combo,
            is_convert,
        } = attrs;

        Self {
            mode: PyGameMode::Mania,
            stars,
            is_convert,
            n_objects: Some(n_objects),
            n_hold_notes: Some(n_hold_notes),
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
            aim_difficult_slider_count,
            speed,
            flashlight,
            slider_factor,
            aim_top_weighted_slider_factor,
            speed_top_weighted_slider_factor,
            speed_note_count,
            aim_difficult_strain_count,
            speed_difficult_strain_count,
            nested_score_per_object,
            legacy_score_base_multiplier,
            maximum_legacy_combo_score,
            hp,
            n_circles,
            n_sliders,
            n_large_ticks,
            n_spinners,
            stamina,
            reading,
            rhythm,
            color,
            n_fruits,
            n_droplets,
            n_tiny_droplets,
            n_objects,
            n_hold_notes,
            ar,
            preempt,
            great_hit_window,
            ok_hit_window,
            meh_hit_window,
            mono_stamina_factor,
            mechanical_difficulty,
            consistency_factor,
            max_combo,
        } = attrs;

        match mode {
            PyGameMode::Osu => {
                if let (
                    Some(aim),
                    Some(aim_difficult_slider_count),
                    Some(speed),
                    Some(flashlight),
                    Some(slider_factor),
                    Some(aim_top_weighted_slider_factor),
                    Some(speed_top_weighted_slider_factor),
                    Some(speed_note_count),
                    Some(aim_difficult_strain_count),
                    Some(speed_difficult_strain_count),
                    Some(nested_score_per_object),
                    Some(legacy_score_base_multiplier),
                    Some(maximum_legacy_combo_score),
                    Some(ar),
                    Some(great_hit_window),
                    Some(ok_hit_window),
                    Some(meh_hit_window),
                    Some(hp),
                    Some(n_circles),
                    Some(n_sliders),
                    Some(n_large_ticks),
                    Some(n_spinners),
                ) = (
                    aim,
                    aim_difficult_slider_count,
                    speed,
                    flashlight,
                    slider_factor,
                    aim_top_weighted_slider_factor,
                    speed_top_weighted_slider_factor,
                    speed_note_count,
                    aim_difficult_strain_count,
                    speed_difficult_strain_count,
                    nested_score_per_object,
                    legacy_score_base_multiplier,
                    maximum_legacy_combo_score,
                    ar,
                    great_hit_window,
                    ok_hit_window,
                    meh_hit_window,
                    hp,
                    n_circles,
                    n_sliders,
                    n_large_ticks,
                    n_spinners,
                ) {
                    return Ok(Self::Osu(OsuDifficultyAttributes {
                        aim,
                        aim_difficult_slider_count,
                        speed,
                        flashlight,
                        slider_factor,
                        aim_top_weighted_slider_factor,
                        speed_top_weighted_slider_factor,
                        speed_note_count,
                        aim_difficult_strain_count,
                        speed_difficult_strain_count,
                        nested_score_per_object,
                        legacy_score_base_multiplier,
                        maximum_legacy_combo_score,
                        ar,
                        great_hit_window,
                        ok_hit_window,
                        meh_hit_window,
                        hp,
                        n_circles,
                        n_sliders,
                        n_large_ticks,
                        n_spinners,
                        stars,
                        max_combo,
                    }));
                }
            }
            PyGameMode::Taiko => {
                if let (
                    Some(stamina),
                    Some(reading),
                    Some(rhythm),
                    Some(color),
                    Some(great_hit_window),
                    Some(ok_hit_window),
                    Some(mono_stamina_factor),
                    Some(mechanical_difficulty),
                    Some(consistency_factor),
                ) = (
                    stamina,
                    reading,
                    rhythm,
                    color,
                    great_hit_window,
                    ok_hit_window,
                    mono_stamina_factor,
                    mechanical_difficulty,
                    consistency_factor,
                ) {
                    return Ok(Self::Taiko(TaikoDifficultyAttributes {
                        stamina,
                        reading,
                        rhythm,
                        color,
                        great_hit_window,
                        ok_hit_window,
                        mono_stamina_factor,
                        mechanical_difficulty,
                        consistency_factor,
                        stars,
                        max_combo,
                        is_convert,
                    }));
                }
            }
            PyGameMode::Catch => {
                if let (Some(preempt), Some(n_fruits), Some(n_droplets), Some(n_tiny_droplets)) =
                    (preempt, n_fruits, n_droplets, n_tiny_droplets)
                {
                    return Ok(Self::Catch(CatchDifficultyAttributes {
                        stars,
                        preempt,
                        n_fruits,
                        n_droplets,
                        n_tiny_droplets,
                        is_convert,
                    }));
                }
            }
            PyGameMode::Mania => {
                if let (Some(n_objects), Some(n_hold_notes)) = (n_objects, n_hold_notes) {
                    return Ok(Self::Mania(ManiaDifficultyAttributes {
                        stars,
                        n_objects,
                        n_hold_notes,
                        max_combo,
                        is_convert,
                    }));
                }
            }
        }

        Err(ParseError::new_err("invalid difficulty attributes"))
    }
}
