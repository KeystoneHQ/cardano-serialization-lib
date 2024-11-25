use crate::protocol_types::Deserialize;
use crate::{DeserializeError, PublicKey, Vkey};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use core2::io::{BufRead, Seek, SeekFrom, Write};

impl cbor_event::se::Serialize for Vkey {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.0.as_bytes())
    }
}

impl Deserialize for Vkey {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(PublicKey(
            crate::chain_crypto::PublicKey::from_binary(raw.bytes()?.as_ref())?,
        )))
    }
}
