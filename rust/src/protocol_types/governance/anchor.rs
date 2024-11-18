use crate::serialization::check_len;
use crate::*;

#[wasm_bindgen]
#[derive(
    Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub struct Anchor {
    pub(crate) anchor_url: URL,
    pub(crate) anchor_data_hash: AnchorDataHash,
}

impl_to_from!(Anchor);

#[wasm_bindgen]
impl Anchor {
    pub fn url(&self) -> URL {
        self.anchor_url.clone()
    }

    pub fn anchor_data_hash(&self) -> AnchorDataHash {
        self.anchor_data_hash.clone()
    }

    pub fn new(anchor_url: &URL, anchor_data_hash: &AnchorDataHash) -> Self {
        Self {
            anchor_url: anchor_url.clone(),
            anchor_data_hash: anchor_data_hash.clone(),
        }
    }
}

impl cbor_event::se::Serialize for Anchor {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.anchor_url.serialize(serializer)?;
        self.anchor_data_hash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for Anchor {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;

            check_len(len, 2, "(anchor_url, anchor_data_hash)")?;

            let anchor_url = URL::deserialize(raw).map_err(|e| e.annotate("anchor_url"))?;

            let anchor_data_hash =
                AnchorDataHash::deserialize(raw).map_err(|e| e.annotate("anchor_data_hash"))?;

            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }

            return Ok(Anchor {
                anchor_url,
                anchor_data_hash,
            });
        })()
        .map_err(|e| e.annotate("Anchor"))
    }
}
