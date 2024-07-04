use std::fmt::{Debug, Formatter, Result as FmtResult};

use pyo3::pyclass;
use rosu_pp::model::mode::GameMode;

#[pyclass(eq, eq_int, name = "GameMode")]
#[derive(Copy, Clone, Default, PartialEq)]
pub enum PyGameMode {
    #[default]
    Osu,
    Taiko,
    Catch,
    Mania,
}

impl Debug for PyGameMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.__pyo3__repr__())
    }
}

impl From<PyGameMode> for GameMode {
    fn from(mode: PyGameMode) -> Self {
        match mode {
            PyGameMode::Osu => Self::Osu,
            PyGameMode::Taiko => Self::Taiko,
            PyGameMode::Catch => Self::Catch,
            PyGameMode::Mania => Self::Mania,
        }
    }
}

impl From<GameMode> for PyGameMode {
    fn from(mode: GameMode) -> Self {
        match mode {
            GameMode::Osu => Self::Osu,
            GameMode::Taiko => Self::Taiko,
            GameMode::Catch => Self::Catch,
            GameMode::Mania => Self::Mania,
        }
    }
}
