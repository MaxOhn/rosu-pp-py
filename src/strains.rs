use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use pyo3::{ffi, pyclass, pymethods, types::PyList, IntoPy, IntoPyPointer, Py, PyObject, Python};
use rosu_pp::{
    catch::CatchStrains, mania::ManiaStrains, osu::OsuStrains, taiko::TaikoStrains, Strains,
};

#[pyclass(name = "Strains")]
#[derive(Debug)]
pub struct PyStrains {
    inner: Strains,
}

impl From<Strains> for PyStrains {
    #[inline]
    fn from(strains: Strains) -> Self {
        Self { inner: strains }
    }
}

impl Display for PyStrains {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut debug = f.debug_struct("Strains");

        macro_rules! debug {
            ( $( $field:ident $( , )? )* ) => {
                debug $( .field(stringify!($field), $field) )*;
            }
        }

        match &self.inner {
            Strains::Osu(strains) => {
                let OsuStrains {
                    section_len,
                    aim,
                    aim_no_sliders,
                    speed,
                    flashlight,
                } = strains;

                debug.field("mode", &0_u8);
                debug!(section_len, aim, aim_no_sliders, speed, flashlight);
            }
            Strains::Taiko(strains) => {
                let TaikoStrains {
                    section_len,
                    color,
                    rhythm,
                    stamina,
                } = strains;

                debug.field("mode", &1_u8);
                debug!(section_len, color, rhythm, stamina);
            }
            Strains::Catch(strains) => {
                let CatchStrains {
                    section_len,
                    movement,
                } = strains;

                debug.field("mode", &2_u8);
                debug!(section_len, movement);
            }
            Strains::Mania(strains) => {
                let ManiaStrains {
                    section_len,
                    strains,
                } = strains;

                debug.field("mode", &3_u8);
                debug!(section_len, strains);
            }
        }

        debug.finish()
    }
}

macro_rules! getters {
    (
        $(
            $mode:ident {
                $( $field:ident ,)*
            },
        )*
    ) => {
        #[pymethods]
        impl PyStrains {
            #[getter]
            fn mode(&self) -> u8 {
                match self.inner {
                    Strains::Osu(_) => 0,
                    Strains::Taiko(_) => 1,
                    Strains::Catch(_) => 2,
                    Strains::Mania(_) => 3,
                }
            }

            #[getter]
            fn section_len(&self) -> f64 {
                match &self.inner {
                    Strains::Osu(strains) => strains.section_len,
                    Strains::Taiko(strains) => strains.section_len,
                    Strains::Catch(strains) => strains.section_len,
                    Strains::Mania(strains) => strains.section_len,
                }
            }

            fn __repr__(&self) -> String {
                self.to_string()
            }

            $(
                $(
                    #[getter]
                    fn $field(&self) -> Option<SliceWrapper<'_>> {
                        if let Strains::$mode(ref attrs) = self.inner {
                            Some(SliceWrapper(&attrs.$field))
                        } else {
                            None
                        }
                    }
                )*
            )*
        }
    };
}

getters! {
    Osu {
        aim,
        aim_no_sliders,
        speed,
        flashlight,
    },
    Taiko {
        color,
        stamina,
        rhythm,
    },
    Catch {
        movement,
    },
    Mania {
        strains,
    },
}

struct SliceWrapper<'i>(&'i [f64]);

impl IntoPy<PyObject> for SliceWrapper<'_> {
    #[inline]
    fn into_py(self, py: Python<'_>) -> PyObject {
        let iter = self.0.iter().map(|e| e.into_py(py));
        let len = self.0.len() as ffi::Py_ssize_t;

        // SAFETY: analogous code to pyo3's `IntoPy` impl for `Vec<T>`
        // https://github.com/PyO3/pyo3/blob/d7b05cbcf5785019a097e496454b924f3e11d94f/src/types/list.rs#L21-L54
        unsafe {
            let ptr = ffi::PyList_New(len);
            let list: Py<PyList> = Py::from_owned_ptr(py, ptr);

            for (item, i) in iter.zip(0..) {
                #[cfg(not(Py_LIMITED_API))]
                ffi::PyList_SET_ITEM(ptr, i, item.into_ptr());
                #[cfg(Py_LIMITED_API)]
                ffi::PyList_SetItem(ptr, i, obj.into_ptr());
            }

            list.into()
        }
    }
}
