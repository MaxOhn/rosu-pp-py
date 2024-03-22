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
                    ( $inner_field:ident ? ) => {
                        if let Some(ref $inner_field) = self.$inner_field {
                            debug.field(stringify!($inner_field), $inner_field);
                        }
                    };

                    ( $inner_field:ident ! ) => {
                        debug.field(stringify!($inner_field), &self.$inner_field);
                    };
                }

                $( debug_field!($field $ty_type); )*

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
