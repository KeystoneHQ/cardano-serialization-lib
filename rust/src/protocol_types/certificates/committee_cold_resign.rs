use crate::*;
use crate::serialization::{
    check_len, deserialize_and_check_index, serialize_and_check_index,
};

#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
#[wasm_bindgen]
pub struct CommitteeColdResign {
    pub(crate) committee_cold_key: StakeCredential,
    pub(crate) anchor: Option<Anchor>,
}

impl_to_from!(CommitteeColdResign);

#[wasm_bindgen]
impl CommitteeColdResign {
    pub fn committee_cold_key(&self) -> StakeCredential {
        self.committee_cold_key.clone()
    }

    pub fn anchor(&self) -> Option<Anchor> {
        self.anchor.clone()
    }

    pub fn new(committee_cold_key: &StakeCredential) -> Self {
        Self {
            committee_cold_key: committee_cold_key.clone(),
            anchor: None,
        }
    }

    pub fn new_with_anchor(committee_cold_key: &StakeCredential, anchor: &Anchor) -> Self {
        Self {
            committee_cold_key: committee_cold_key.clone(),
            anchor: Some(anchor.clone()),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.committee_cold_key.has_script_hash()
    }
}

impl cbor_event::se::Serialize for CommitteeColdResign {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        let proposal_index = certificate_index_names::CertificateIndexNames::CommitteeColdResign.to_u64();
        serialize_and_check_index(serializer, proposal_index, "CommitteeColdResign")?;

        self.committee_cold_key.serialize(serializer)?;
        self.anchor.serialize_nullable(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(CommitteeColdResign);

impl DeserializeEmbeddedGroup for CommitteeColdResign {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 3, "(cert_index, committee_cold_key, anchor)")?;

        let cert_index = certificate_index_names::CertificateIndexNames::CommitteeColdResign.to_u64();
        deserialize_and_check_index(raw, cert_index, "cert_index")?;

        let committee_cold_key =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("committee_cold_key"))?;
        let anchor = Anchor::deserialize_nullable(raw).map_err(|e| e.annotate("anchor"))?;

        Ok(CommitteeColdResign { committee_cold_key, anchor })
    }
}
