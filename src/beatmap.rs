use pyo3::{exceptions::PyTypeError, pyclass, pymethods, types::PyDict, PyResult};
use rosu_pp::Beatmap;

use crate::error::{ErrorExt, KwargsError, ParseError};

#[pyclass(name = "Beatmap")]
pub struct PyBeatmap {
    pub(crate) inner: Beatmap,
}

#[pymethods]
impl PyBeatmap {
    #[new]
    #[args(kwargs = "**")]
    fn new(kwargs: Option<&PyDict>) -> PyResult<Self> {
        let kwargs = match kwargs {
            Some(kwargs) => kwargs,
            None => {
                return Err(KwargsError::new_err(
                    "'Beatmap' constructor requires kwargs",
                ))
            }
        };

        if let Some(arg) = kwargs.get_item("path") {
            let path = arg
                .extract::<&str>()
                .map_err(|_| PyTypeError::new_err("kwarg 'path': must be a string"))?;

            let map = Beatmap::from_path(path)
                .map_err(|e| ParseError::new_err(e.unwind("Failed to parse beatmap")))?;

            Self::new_with_attrs(map, kwargs)
        } else if let Some(arg) = kwargs.get_item("content") {
            if let Ok(content) = arg.extract::<&str>() {
                Self::new_from_bytes(content.as_bytes(), kwargs)
            } else if let Ok(bytes) = arg.extract::<&[u8]>() {
                Self::new_from_bytes(bytes, kwargs)
            } else {
                Err(PyTypeError::new_err(
                    "kwarg 'content': must be a string or a bytearray",
                ))
            }
        } else if let Some(arg) = kwargs.get_item("bytes") {
            let bytes = arg
                .extract::<&[u8]>()
                .map_err(|_| PyTypeError::new_err("kwarg 'bytes': must be a bytearray"))?;

            Self::new_from_bytes(bytes, kwargs)
        } else {
            Err(KwargsError::new_err(
                "kwargs must include 'path', 'content', or 'bytes'",
            ))
        }
    }

    fn set_ar(&mut self, ar: f32) {
        self.inner.ar = ar;
    }

    fn set_cs(&mut self, cs: f32) {
        self.inner.cs = cs;
    }

    fn set_hp(&mut self, hp: f32) {
        self.inner.hp = hp;
    }

    fn set_od(&mut self, od: f32) {
        self.inner.od = od;
    }
}

impl PyBeatmap {
    fn new_from_bytes(bytes: &[u8], kwargs: &PyDict) -> PyResult<Self> {
        let map = Beatmap::from_bytes(bytes)
            .map_err(|e| ParseError::new_err(e.unwind("Failed to parse beatmap")))?;

        Self::new_with_attrs(map, kwargs)
    }

    fn new_with_attrs(mut map: Beatmap, kwargs: &PyDict) -> PyResult<Self> {
        macro_rules! parse_attr {
            ( $( $name:ident ),*) => {
                $(
                    if let Some(arg) = kwargs.get_item(stringify!($name)) {
                        let value = arg.extract::<f32>().map_err(|_| {
                            PyTypeError::new_err(concat!(
                                "kwarg '",
                                stringify!($name),
                                "': must be a real number"
                            ))
                        })?;

                        map.$name = value;
                    }
                )*
            };
        }

        parse_attr!(ar, cs, hp, od);

        Ok(Self { inner: map })
    }
}
