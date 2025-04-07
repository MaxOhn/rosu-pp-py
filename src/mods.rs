use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Deref,
};

use pyo3::{
    impl_::frompyobject::{
        extract_struct_field, extract_tuple_struct_field, failed_to_extract_enum,
    },
    intern,
    types::{
        iter::BoundDictIterator, PyAnyMethods, PyDict, PyDictMethods, PyList, PyString,
        PyStringMethods,
    },
    Bound, FromPyObject, Py, PyAny, PyResult, Python,
};
use rosu_mods::{
    serde::GameModSeed, GameMode, GameMods as GameModsLazer, GameModsIntermode, GameModsLegacy,
};
use serde::de::{
    value::{BorrowedStrDeserializer, CowStrDeserializer, MapAccessDeserializer, U32Deserializer},
    DeserializeSeed, Deserializer, Error as DeError, MapAccess, Unexpected, Visitor,
};

use crate::error::ParseError;

#[derive(Clone)]
pub enum PyGameMods {
    Lazer(GameModsLazer),
    Intermode(GameModsIntermode),
    Legacy(GameModsLegacy),
}

impl PyGameMods {
    pub(crate) fn extract<'py>(
        mods: Option<&Py<PyAny>>,
        mode: rosu_pp::model::mode::GameMode,
        py: Python<'py>,
    ) -> PyResult<Self> {
        let Some(mods) = mods else {
            return Ok(Self::default());
        };

        let obj = mods.bind(py);
        let mode = GameMode::from(mode as u8);

        let errors = [
            match extract_tuple_struct_field(obj, "PyInt", 0) {
                Ok(bits) => return Ok(Self::Legacy(GameModsLegacy::from_bits(bits))),
                Err(err) => err,
            },
            match extract_tuple_struct_field::<Bound<'py, PyString>>(obj, "PyString", 0) {
                Ok(acronyms) => match acronyms.to_str() {
                    Ok(acronyms) => {
                        let intermode = GameModsIntermode::from_acronyms(acronyms);

                        let mods = match intermode.checked_bits() {
                            Some(bits) => Self::Legacy(GameModsLegacy::from_bits(bits)),
                            None => Self::Intermode(intermode),
                        };

                        return Ok(mods);
                    }
                    Err(err) => err,
                },
                Err(err) => err,
            },
            match extract_tuple_struct_field::<PyGameMod<'py>>(obj, "PyGameMod", 0) {
                Ok(gamemod) => {
                    let seed = GameModSeed::Mode {
                        mode,
                        deny_unknown_fields: false,
                    };

                    match seed.deserialize(&gamemod) {
                        Ok(gamemod) => return Ok(Self::Lazer(gamemod.into())),
                        Err(DeserializeError(err)) => ParseError::new_err(err),
                    }
                }
                Err(err) => err,
            },
            match extract_tuple_struct_field::<Bound<'py, PyList>>(obj, "PyList", 0) {
                Ok(list) => {
                    let seed = GameModSeed::Mode {
                        mode,
                        deny_unknown_fields: false,
                    };

                    let res = list
                        .try_iter()?
                        .try_fold(GameModsLazer::new(), |mut mods, item| {
                            let res = match item?.extract::<PyGameModUnion<'_>>()? {
                                PyGameModUnion::Mod(gamemod) => seed.deserialize(&gamemod),
                                PyGameModUnion::Acronym(acronym) => seed.deserialize(
                                    CowStrDeserializer::new(acronym.to_string_lossy()),
                                ),
                                PyGameModUnion::Bits(bits) => {
                                    seed.deserialize(U32Deserializer::new(bits))
                                }
                            };

                            match res {
                                Ok(gamemod) => mods.insert(gamemod),
                                Err(DeserializeError(err)) => return Err(ParseError::new_err(err)),
                            }

                            Ok(mods)
                        });

                    match res {
                        Ok(mods) => return Ok(Self::Lazer(mods)),
                        Err(err) => err,
                    }
                }
                Err(err) => err,
            },
        ];

        Err(failed_to_extract_enum(
            obj.py(),
            "PyGameMods",
            &["Legacy", "Intermode", "GameMods", "GameMods"],
            &["int", "str", "GameMod", "List[GameMod | str | int]"],
            &errors,
        ))
    }
}

impl Default for PyGameMods {
    fn default() -> Self {
        Self::Legacy(GameModsLegacy::NoMod)
    }
}

struct PyGameMod<'py> {
    acronym: Bound<'py, PyString>,
    settings: Option<Bound<'py, PyDict>>,
}

impl<'py> FromPyObject<'py> for PyGameMod<'py> {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        let py = obj.py();
        let dict: Bound<'_, PyDict> = obj.extract()?;

        // Force a `KeyError` if `acronym` is missing
        let acronym = PyAnyMethods::get_item(dict.deref(), intern!(py, "acronym"))?;
        let settings = dict.get_item(intern!(py, "settings"))?;

        Ok(PyGameMod {
            acronym: extract_struct_field(&acronym, "PyGameMod", "acronym")?,
            settings: settings
                .as_ref()
                .map(PyAnyMethods::extract::<Option<Bound<'_, PyDict>>>)
                .transpose()?
                .flatten(),
        })
    }
}

#[derive(FromPyObject)]
enum PyGameModUnion<'py> {
    Mod(PyGameMod<'py>),
    Acronym(Bound<'py, PyString>),
    Bits(u32),
}

#[derive(Debug)]
struct DeserializeError(String);

impl DeError for DeserializeError {
    fn custom<T: Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.0)
    }
}

impl StdError for DeserializeError {}

impl<'de> Deserializer<'de> for &'de PyGameMod<'de> {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_bool<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        _: usize,
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(PyGameModMap::Full(self))
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

enum PyGameModMap<'py> {
    Full(&'py PyGameMod<'py>),
    Settings(&'py Bound<'py, PyDict>),
    Done,
}

impl<'de> MapAccess<'de> for PyGameModMap<'de> {
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        let key = match self {
            PyGameModMap::Full(_) => "acronym",
            PyGameModMap::Settings(_) => "settings",
            PyGameModMap::Done => return Ok(None),
        };

        seed.deserialize(BorrowedStrDeserializer::new(key))
            .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self {
            PyGameModMap::Full(gamemod) => {
                let acronym = gamemod.acronym.to_string_lossy();
                let res = seed.deserialize(CowStrDeserializer::new(acronym));
                *self = gamemod.settings.as_ref().map_or(Self::Done, Self::Settings);

                res
            }
            PyGameModMap::Settings(dict) => {
                let access = DictAccess {
                    iter: dict.iter(),
                    next_value: None,
                };

                let res = seed.deserialize(MapAccessDeserializer::new(access));
                *self = Self::Done;

                res
            }
            PyGameModMap::Done => unimplemented!(),
        }
    }
}

struct DictAccess<'py> {
    iter: BoundDictIterator<'py>,
    next_value: Option<PyValue<'py>>,
}

impl<'de> MapAccess<'de> for DictAccess<'de> {
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        debug_assert!(self.next_value.is_none());

        match self.iter.next() {
            Some((key, value)) => {
                let key: Bound<'_, PyString> = key.extract().map_err(DeError::custom)?;
                let value: PyValue<'_> = value.extract().map_err(DeError::custom)?;
                self.next_value = Some(value);

                seed.deserialize(PyValue::String(key)).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(self.next_value.take().unwrap())
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
    }
}

#[derive(FromPyObject)]
enum PyValue<'py> {
    Bool(bool),
    Number(f64),
    String(Bound<'py, PyString>),
}

impl<'de> Deserializer<'de> for PyValue<'de> {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            PyValue::Bool(v) => visitor.visit_bool(v),
            PyValue::Number(v) => visitor.visit_f64(v),
            PyValue::String(v) => visitor.visit_string(v.to_string_lossy().into_owned()),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Self::Bool(v) => visitor.visit_bool(v),
            Self::Number(v) => Err(DeError::invalid_type(Unexpected::Float(v), &visitor)),
            Self::String(v) => Err(DeError::invalid_type(
                Unexpected::Str(v.to_string_lossy().as_ref()),
                &visitor,
            )),
        }
    }

    fn deserialize_i8<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Self::Bool(v) => Err(DeError::invalid_type(Unexpected::Bool(v), &visitor)),
            Self::Number(v) => visitor.visit_f64(v),
            Self::String(v) => Err(DeError::invalid_type(
                Unexpected::Str(v.to_string_lossy().as_ref()),
                &visitor,
            )),
        }
    }

    fn deserialize_char<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Self::Bool(v) => Err(DeError::invalid_type(Unexpected::Bool(v), &visitor)),
            Self::Number(v) => Err(DeError::invalid_type(Unexpected::Float(v), &visitor)),
            Self::String(v) => visitor.visit_str(v.to_string_lossy().as_ref()),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Self::Bool(v) => Err(DeError::invalid_type(Unexpected::Bool(v), &visitor)),
            Self::Number(v) => Err(DeError::invalid_type(Unexpected::Float(v), &visitor)),
            Self::String(v) => visitor.visit_string(v.to_string_lossy().into_owned()),
        }
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        _: usize,
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}
