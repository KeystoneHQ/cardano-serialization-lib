use crate::protocol_types::Deserialize;
use crate::{DeserializeError, DeserializeFailure, KESSignature};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;

use core2 as std;
impl cbor_event::se::Serialize for KESSignature {
    fn serialize<'se, W: core2::io::Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for KESSignature {
    fn deserialize<R: core2::io::BufRead>(
        raw: &mut Deserializer<R>,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<Self, DeserializeError> {
            let bytes = raw.bytes()?;
            if bytes.len() != Self::BYTE_COUNT {
                return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                    Self::BYTE_COUNT as u64,
                    cbor_event::Len::Len(bytes.len() as u64),
                    "hash length",
                ))
                .into());
            }
            Ok(KESSignature(bytes))
        })()
        .map_err(|e| e.annotate("KESSignature"))
    }
}
