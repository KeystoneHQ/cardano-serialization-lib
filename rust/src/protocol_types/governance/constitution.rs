use crate::*;
use serialization::check_len;

#[derive(
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[wasm_bindgen]
pub struct Constitution {
    pub(crate) anchor: Anchor,
    pub(crate) script_hash: Option<ScriptHash>,
}

impl_to_from!(Constitution);

#[wasm_bindgen]
impl Constitution {
    pub fn anchor(&self) -> Anchor {
        self.anchor.clone()
    }

    pub fn script_hash(&self) -> Option<ScriptHash> {
        self.script_hash.clone()
    }

    pub fn new(anchor: &Anchor) -> Self {
        Self {
            anchor: anchor.clone(),
            script_hash: None,
        }
    }

    pub fn new_with_script_hash(anchor: &Anchor, script_hash: &ScriptHash) -> Self {
        Self {
            anchor: anchor.clone(),
            script_hash: Some(script_hash.clone()),
        }
    }
}



impl Serialize for Constitution {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.anchor.serialize(serializer)?;
        self.script_hash.serialize_nullable(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(Constitution);

impl DeserializeEmbeddedGroup for Constitution {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 2, "(anchor, scripthash / null)")?;
        let anchor = Anchor::deserialize(raw)?;
        let script_hash = ScriptHash::deserialize_nullable(raw)?;

        Ok(Constitution {
            anchor,
            script_hash,
        })
    }
}