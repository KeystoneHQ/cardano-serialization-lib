use crate::*;

pub trait NoneOrEmpty {
    fn is_none_or_empty(&self) -> bool;
}

impl<T: NoneOrEmpty> NoneOrEmpty for &T {
    fn is_none_or_empty(&self) -> bool {
        (*self).is_none_or_empty()
    }
}

impl<T: NoneOrEmpty> NoneOrEmpty for Option<T> {
    fn is_none_or_empty(&self) -> bool {
        self.is_none() || self.as_ref().unwrap().is_none_or_empty()
    }
}

pub trait DeserializeNullable {
    fn deserialize_nullable<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
    ) -> Result<Option<Self>, DeserializeError>
    where
        Self: Sized;
}

impl<T: Deserialize> DeserializeNullable for T {
    fn deserialize_nullable<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
    ) -> Result<Option<Self>, DeserializeError>
    where
        Self: Sized,
    {
        if raw.cbor_type()? == CBORType::Special {
            if raw.special()? != CBORSpecial::Null {
                return Err(DeserializeFailure::ExpectedNull.into());
            }
            Ok(None)
        } else {
            Ok(Some(T::deserialize(raw)?))
        }
    }
}

pub trait SerializeNullable {
    fn serialize_nullable<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>>;
}

impl<T: Serialize> SerializeNullable for Option<T> {
    fn serialize_nullable<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>> {
        match self {
            Some(x) => x.serialize(serializer),
            None => serializer.write_special(CBORSpecial::Null),
        }
    }
}