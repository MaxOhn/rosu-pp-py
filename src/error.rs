use std::{error::Error, fmt::Write};

use pyo3::{create_exception, exceptions::PyException};

create_exception!(rosu_pp_py, KwargsError, PyException);
create_exception!(rosu_pp_py, ParseError, PyException);

pub trait ErrorExt {
    fn unwind(&self, cause: &str) -> String;
}

impl<T: Error> ErrorExt for T {
    fn unwind(&self, cause: &str) -> String {
        let mut e = self as &dyn Error;
        let mut content = format!("{cause}\n  - caused by: {e}");

        while let Some(src) = e.source() {
            let _ = write!(content, "\n  - caused by: {src}");
            e = src;
        }

        content
    }
}
