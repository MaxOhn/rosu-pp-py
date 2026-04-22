use std::fmt::{Debug, Formatter, Result as FmtResult};

use pyo3::pyclass;
use rosu_pp::model::mode::GameMode;

#[pyclass(eq, eq_int, name = "GameMode", from_py_object)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use rosu_pp::model::mode::GameMode as RustGameMode;

    #[test]
    fn test_py_to_rust_conversion() {
        assert_eq!(RustGameMode::from(PyGameMode::Osu), RustGameMode::Osu);
        assert_eq!(RustGameMode::from(PyGameMode::Taiko), RustGameMode::Taiko);
        assert_eq!(RustGameMode::from(PyGameMode::Catch), RustGameMode::Catch);
        assert_eq!(RustGameMode::from(PyGameMode::Mania), RustGameMode::Mania);
    }

    #[test]
    fn test_rust_to_py_conversion() {
        assert_eq!(PyGameMode::from(RustGameMode::Osu), PyGameMode::Osu);
        assert_eq!(PyGameMode::from(RustGameMode::Taiko), PyGameMode::Taiko);
        assert_eq!(PyGameMode::from(RustGameMode::Catch), PyGameMode::Catch);
        assert_eq!(PyGameMode::from(RustGameMode::Mania), PyGameMode::Mania);
    }

    #[test]
    fn test_roundtrip_conversion() {
        for mode in [
            RustGameMode::Osu,
            RustGameMode::Taiko,
            RustGameMode::Catch,
            RustGameMode::Mania,
        ] {
            let py = PyGameMode::from(mode);
            let back = RustGameMode::from(py);
            assert_eq!(mode, back);
        }
    }

    #[test]
    fn test_equality() {
        assert_eq!(PyGameMode::Osu, PyGameMode::Osu);
        assert_eq!(PyGameMode::Taiko, PyGameMode::Taiko);
        assert!(PyGameMode::Osu != PyGameMode::Taiko);
    }

    #[test]
    fn test_default() {
        let default = PyGameMode::default();
        assert_eq!(default, PyGameMode::Osu);
    }

    #[test]
    fn test_copy_clone() {
        let mode = PyGameMode::Taiko;
        let copied = mode;
        let cloned = mode.clone();
        assert_eq!(mode, copied);
        assert_eq!(mode, cloned);
    }
}
