use crate::{Number, Value, Map};
use serde::de::{self, Visitor, Deserializer, Deserialize, MapAccess, SeqAccess};
use std::fmt;


impl<'de> Deserialize<'de> for Value<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Value<'de>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}
struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an JSONesque value")
    }

    /****************** unit ******************/
    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    /****************** bool ******************/
    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(Value::Bool(value))
    }

    /****************** Option ******************/
    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_some<D>(self, deserializer: D)  -> Result<Self::Value, D::Error> where
        D: Deserializer<'de>, {
        deserializer.deserialize_any(self)
    }

    /****************** enum ******************/
    /*
    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error> where
        A: EnumAccess<'de>,
    {
    }
     */

    /****************** i64 ******************/
    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(Number::I64(value as i64)))
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(Number::I64(value as i64)))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(Number::I64(value as i64)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(Number::I64(value)))
    }

    /****************** u64 ******************/

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(Number::I64(value as i64)))
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(Number::I64(value as i64)))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(Number::I64(value as i64)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        use std::i64;
        if value <= i64::MAX as u64 {
            Ok(Value::Number(Number::I64(value as i64)))
        } else {
            Err(E::custom(format!("Integer out of range: {}", value)))
        }
    }

    /****************** f64 ******************/

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(Number::F64(value as f64)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(Number::F64(value)))
    }

    /****************** stringy stuff ******************/
    fn visit_char<E>(self, _value: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        unimplemented!()
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::String(value))
    }

    /*
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        'a: 'de
        E: de::Error,
    {
        Ok(Value::String(value))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::String(&value))
    }
     */

    /****************** byte stuff ******************/

    /*
    fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::String(value))
    }

    fn visit_str<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
    'a: 'de
        E: de::Error,
    {
      Ok(Value::String(value))
    }

    fn visit_string<E>(self, value: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
      Ok(Value::String(&value))
    }
     */
    /****************** nexted stuff ******************/

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where
        A: MapAccess<'de>,
    {
        let size = map.size_hint().unwrap_or_default();

        let mut m = Map::with_capacity(size);
        while let Some(k) = map.next_key()? {
            let v = map.next_value()?;
            m.insert(k, v);
        }
        Ok(Value::Map(m))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where
        A: SeqAccess<'de>,
    {
        let size = seq.size_hint().unwrap_or_default();

        let mut v = Vec::with_capacity(size);
        while let Some(e) = seq.next_element()? {
            v.push(e);
        }
        Ok(Value::Array(v))
    }
}
