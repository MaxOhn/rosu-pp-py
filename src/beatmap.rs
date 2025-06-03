use std::{error::Error as StdError, fmt::Write};

use pyo3::{
    exceptions::PyTypeError,
    pyclass, pymethods,
    types::{PyAnyMethods, PyDict},
    Bound, Py, PyAny, PyResult, Python,
};
use rosu_pp::{
    model::{hit_object::HitObjectKind, mode::GameMode},
    Beatmap,
};

use crate::{
    error::{ArgsError, ConvertError, ParseError},
    mode::PyGameMode,
    mods::PyGameMods,
};

#[pyclass(name = "Beatmap")]
pub struct PyBeatmap {
    pub(crate) inner: Beatmap,
}

#[pymethods]
impl PyBeatmap {
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn new(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        let Some(kwargs) = kwargs else {
            return Err(ArgsError::new_err(
                "kwarg 'path', 'bytes', or 'content' must be specified",
            ));
        };

        let mut map_res = None;

        for (key, value) in kwargs {
            match key.extract()? {
                "path" => {
                    let path = value
                        .extract::<&str>()
                        .map_err(|_| PyTypeError::new_err("kwarg 'path': must be a str"))?;

                    map_res = Some(Beatmap::from_path(path));
                }
                "content" => {
                    let bytes = if let Ok(content) = value.extract::<&str>() {
                        content.as_bytes()
                    } else if let Ok(bytes) = value.extract::<&[u8]>() {
                        bytes
                    } else {
                        return Err(PyTypeError::new_err(
                            "kwarg 'content': must be a str or a bytearray",
                        ));
                    };

                    map_res = Some(Beatmap::from_bytes(bytes));
                }
                "bytes" => {
                    let bytes = value
                        .extract::<&[u8]>()
                        .map_err(|_| PyTypeError::new_err("kwarg 'bytes': must be a bytearray"))?;

                    map_res = Some(Beatmap::from_bytes(bytes));
                }
                kwarg => {
                    let err = format!(
                        "unexpected kwarg '{kwarg}': expected 'path', \
                        'content', or 'bytes'"
                    );

                    return Err(ArgsError::new_err(err));
                }
            }
        }

        let map = match map_res {
            Some(Ok(map)) => map,
            Some(Err(err)) => {
                let mut e = &err as &dyn StdError;
                let mut content = format!("Failed to parse beatmap\n  - caused by: {e}");

                while let Some(src) = e.source() {
                    let _ = write!(content, "\n  - caused by: {src}");
                    e = src;
                }

                return Err(ParseError::new_err(content));
            }
            None => {
                return Err(ArgsError::new_err(
                    "kwarg 'path', 'bytes', or 'content' must be specified",
                ))
            }
        };

        Ok(Self { inner: map })
    }

    #[pyo3(signature = (mode, mods=None))]
    fn convert(
        &mut self,
        mode: PyGameMode,
        mods: Option<Py<PyAny>>,
        py: Python<'_>,
    ) -> PyResult<()> {
        let mods = match PyGameMods::extract(mods.as_ref(), mode.into(), py) {
            Ok(PyGameMods::Lazer(mods)) => mods.into(),
            Ok(PyGameMods::Intermode(mods)) => mods.into(),
            Ok(PyGameMods::Legacy(mods)) => mods.into(),
            Err(err) => return Err(err),
        };

        let mode = GameMode::from(mode);

        if let Err(err) = self.inner.convert_mut(mode, &mods) {
            return Err(ConvertError::new_err(err.to_string()));
        }

        Ok(())
    }

    #[pyo3()]
    fn is_suspicious(&self) -> bool {
        self.inner.check_suspicion().is_err()
    }

    #[getter]
    fn bpm(&self) -> f64 {
        self.inner.bpm()
    }

    #[getter]
    fn version(&self) -> i32 {
        self.inner.version
    }

    #[getter]
    fn is_convert(&self) -> bool {
        self.inner.is_convert
    }

    #[getter]
    fn stack_leniency(&self) -> f32 {
        self.inner.stack_leniency
    }

    #[getter]
    fn ar(&self) -> f32 {
        self.inner.ar
    }

    #[getter]
    fn cs(&self) -> f32 {
        self.inner.cs
    }

    #[getter]
    fn hp(&self) -> f32 {
        self.inner.hp
    }

    #[getter]
    fn od(&self) -> f32 {
        self.inner.od
    }

    #[getter]
    fn slider_multiplier(&self) -> f64 {
        self.inner.slider_multiplier
    }

    #[getter]
    fn slider_tick_rate(&self) -> f64 {
        self.inner.slider_tick_rate
    }

    #[getter]
    pub fn mode(&self) -> PyGameMode {
        PyGameMode::from(self.inner.mode)
    }

    #[getter]
    pub fn n_breaks(&self) -> usize {
        self.inner.breaks.len()
    }

    #[getter]
    pub fn n_objects(&self) -> usize {
        self.inner.hit_objects.len()
    }

    #[getter]
    pub fn n_circles(&self) -> usize {
        self.inner
            .hit_objects
            .iter()
            .filter(|h| h.is_circle())
            .count()
    }

    #[getter]
    pub fn n_sliders(&self) -> usize {
        self.inner
            .hit_objects
            .iter()
            .filter(|h| h.is_slider())
            .count()
    }

    #[getter]
    pub fn n_spinners(&self) -> usize {
        self.inner
            .hit_objects
            .iter()
            .filter(|h| h.is_spinner())
            .count()
    }

    #[getter]
    pub fn n_holds(&self) -> usize {
        self.inner
            .hit_objects
            .iter()
            .filter(|h| matches!(h.kind, HitObjectKind::Hold(_)))
            .count()
    }
}
