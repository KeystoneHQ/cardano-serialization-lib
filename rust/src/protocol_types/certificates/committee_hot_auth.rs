use crate::serialization::{check_len, deserialize_and_check_index, serialize_and_check_index};
use crate::*;

#[derive(
    Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[wasm_bindgen]
pub struct CommitteeHotAuth {
    pub(crate) committee_cold_key: StakeCredential,
    pub(crate) committee_hot_key: StakeCredential,
}

impl_to_from!(CommitteeHotAuth);

#[wasm_bindgen]
impl CommitteeHotAuth {
    pub fn committee_cold_key(&self) -> StakeCredential {
        self.committee_cold_key.clone()
    }

    pub fn committee_hot_key(&self) -> StakeCredential {
        self.committee_hot_key.clone()
    }

    pub fn new(committee_cold_key: &StakeCredential, committee_hot_key: &StakeCredential) -> Self {
        Self {
            committee_cold_key: committee_cold_key.clone(),
            committee_hot_key: committee_hot_key.clone(),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.committee_cold_key.has_script_hash()
    }
}

impl cbor_event::se::Serialize for CommitteeHotAuth {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;

        let proposal_index =
            certificate_index_names::CertificateIndexNames::CommitteeHotAuth.to_u64();
        serialize_and_check_index(serializer, proposal_index, "CommitteeHotAuth")?;

        self.committee_cold_key.serialize(serializer)?;
        self.committee_hot_key.serialize(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(CommitteeHotAuth);

impl DeserializeEmbeddedGroup for CommitteeHotAuth {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(
            len,
            3,
            "(cert_index, committee_cold_key, committee_hot_key)",
        )?;

        let cert_index = certificate_index_names::CertificateIndexNames::CommitteeHotAuth.to_u64();
        deserialize_and_check_index(raw, cert_index, "cert_index")?;

        let committee_cold_key =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("committee_cold_key"))?;

        let committee_hot_key =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("committee_hot_key"))?;

        return Ok(CommitteeHotAuth {
            committee_cold_key,
            committee_hot_key,
        });
    }
}
