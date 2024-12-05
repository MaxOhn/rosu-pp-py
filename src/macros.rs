pub struct BoolFormatter(pub bool);

impl std::fmt::Debug for BoolFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.0 { "True" } else { "False" })
    }
}

macro_rules! define_class {
    (
        #[pyclass(name = $py_name:literal $(, $py_meta:meta)* )]
        $( #[ $struct_meta:meta ] )*
        $struct_vis:vis struct $name:ident {
            $( $field_vis:vis $field:ident: $ty:ident $ty_type:tt , )*
        }
    ) => {
        #[pyclass(name = $py_name $(, $py_meta )* )]
        $( #[ $struct_meta ] )*
        $struct_vis struct $name {
            $(
                #[pyo3(get)]
                $field_vis $field: define_class!(@EXPAND_TY $ty $ty_type),
            )*
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let mut debug = f.debug_struct($py_name);

                macro_rules! debug_field {
                    ( $inner_field:ident: $field_ty:ident ? ) => {
                        if let Some(ref $inner_field) = self.$inner_field {
                            debug.field(stringify!($inner_field), debug_field!(@VALUE $field_ty: $inner_field));
                        }
                    };

                    ( $inner_field:ident: $field_ty:ident ! ) => {
                        let field = &self.$inner_field;
                        debug.field(stringify!($inner_field), debug_field!(@VALUE $field_ty: field));
                    };

                    ( @VALUE bool: $value:tt ) => {
                        &crate::macros::BoolFormatter( *$value )
                    };

                    ( @VALUE $field_ty:ident: $value:tt ) => {
                        &$value
                    };
                }

                $( debug_field!($field: $ty $ty_type); )*

                debug.finish()
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(self, f)
            }
        }

        #[pyo3::pymethods]
        impl $name {
            fn __repr__(&self) -> String {
                self.to_string()
            }
        }
    };

    ( @EXPAND_TY $ty:ident ! ) => {
        $ty
    };

    ( @EXPAND_TY $ty:ident ? ) => {
        Option<$ty>
    };
}

macro_rules! extract_args {
    ( $this:ident . $key:ident = $value:ident {
        $( $field:ident: $expected:literal, )+
    } ) => {
        match $key.extract()? {
            $(
                stringify!($field) => {
                    $this.$field = $value
                        .extract()
                        .map_err(|_| PyTypeError::new_err(concat!(
                            "kwarg '",
                            stringify!($field),
                            "': must be ",
                            $expected,
                        )))?
                }
            ),*
            kwarg => {
                return Err(ArgsError::new_err(extract_args!(
                    @ERR kwarg: $( $field ),*
                )));
            }
        }
    };
    (@ERR $kwarg:ident: $first_field:ident $(, $field:ident )*) => {
        format!(concat!(
            "unexpected kwarg '{}': expected ",
            stringify!($first_field),
            $( ", ", stringify!($field), )*
        ), $kwarg)
    };
}
