use crate::serialization::utils::is_break_tag;
use crate::*;

impl cbor_event::se::Serialize for TransactionBodies {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TransactionBodies {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "TransactionBodies")? {
                    break;
                }
                arr.push(TransactionBody::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("TransactionBodies"))?;
        Ok(Self(arr))
    }
}
