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

        let mut map = None;
        let mut ar = None;
        let mut cs = None;
        let mut hp = None;
        let mut od = None;

        for (key, value) in kwargs.iter() {
            match key.extract()? {
                "path" => {
                    let path = value
                        .extract::<&str>()
                        .map_err(|_| PyTypeError::new_err("kwarg 'path': must be a string"))?;

                    let parsed = Beatmap::from_path(path)
                        .map_err(|e| ParseError::new_err(e.unwind("Failed to parse beatmap")))?;

                    map = Some(parsed);
                }
                "content" => {
                    let bytes = if let Ok(content) = value.extract::<&str>() {
                        content.as_bytes()
                    } else if let Ok(bytes) = value.extract::<&[u8]>() {
                        bytes
                    } else {
                        return Err(PyTypeError::new_err(
                            "kwarg 'content': must be a string or a bytearray",
                        ));
                    };

                    let parsed = Beatmap::from_bytes(bytes)
                        .map_err(|e| ParseError::new_err(e.unwind("Failed to parse beatmap")))?;

                    map = Some(parsed);
                }
                "bytes" => {
                    let bytes = value
                        .extract::<&[u8]>()
                        .map_err(|_| PyTypeError::new_err("kwarg 'bytes': must be a bytearray"))?;

                    let parsed = Beatmap::from_bytes(bytes)
                        .map_err(|e| ParseError::new_err(e.unwind("Failed to parse beatmap")))?;

                    map = Some(parsed);
                }
                "ar" => {
                    let value = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'ar': must be a real number"))?;

                    ar = Some(value);
                }
                "cs" => {
                    let value = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'cs': must be a real number"))?;

                    cs = Some(value);
                }
                "hp" => {
                    let value = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'hp': must be a real number"))?;

                    hp = Some(value);
                }
                "od" => {
                    let value = value
                        .extract()
                        .map_err(|_| PyTypeError::new_err("kwarg 'od': must be a real number"))?;

                    od = Some(value);
                }
                kwarg => {
                    let err = format!(
                        "unexpected kwarg '{kwarg}': expected 'path', \n\
                        'content', 'bytes', 'ar', 'cs', 'hp', or 'od'"
                    );

                    return Err(KwargsError::new_err(err));
                }
            }
        }

        let mut map = map.ok_or_else(|| {
            KwargsError::new_err("kwargs must include 'path', 'content', or 'bytes'")
        })?;

        if let Some(ar) = ar {
            map.ar = ar;
        }

        if let Some(cs) = cs {
            map.cs = cs;
        }

        if let Some(hp) = hp {
            map.hp = hp;
        }

        if let Some(od) = od {
            map.od = od;
        }

        Ok(Self { inner: map })
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
