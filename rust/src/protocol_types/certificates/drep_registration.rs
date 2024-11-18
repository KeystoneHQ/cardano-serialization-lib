use crate::serialization::{check_len, deserialize_and_check_index, serialize_and_check_index};
use crate::*;

#[derive(
    Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[wasm_bindgen]
pub struct DrepRegistration {
    pub(crate) voting_credential: StakeCredential,
    pub(crate) coin: Coin,
    pub(crate) anchor: Option<Anchor>,
}

impl_to_from!(DrepRegistration);

#[wasm_bindgen]
impl DrepRegistration {
    pub fn voting_credential(&self) -> StakeCredential {
        self.voting_credential.clone()
    }

    pub fn coin(&self) -> Coin {
        self.coin.clone()
    }

    pub fn anchor(&self) -> Option<Anchor> {
        self.anchor.clone()
    }

    pub fn new(voting_credential: &StakeCredential, coin: &Coin) -> Self {
        Self {
            voting_credential: voting_credential.clone(),
            coin: coin.clone(),
            anchor: None,
        }
    }

    pub fn new_with_anchor(
        voting_credential: &StakeCredential,
        coin: &Coin,
        anchor: &Anchor,
    ) -> Self {
        Self {
            voting_credential: voting_credential.clone(),
            coin: coin.clone(),
            anchor: Some(anchor.clone()),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.voting_credential.has_script_hash()
    }
}

impl cbor_event::se::Serialize for DrepRegistration {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;

        let proposal_index =
            certificate_index_names::CertificateIndexNames::DrepRegistration.to_u64();
        serialize_and_check_index(serializer, proposal_index, "DrepRegistration")?;

        self.voting_credential.serialize(serializer)?;
        self.coin.serialize(serializer)?;
        match &self.anchor {
            Some(anchor) => anchor.serialize(serializer),
            None => serializer.write_special(CBORSpecial::Null),
        }?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(DrepRegistration);

impl DeserializeEmbeddedGroup for DrepRegistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(
            len,
            4,
            "(cert_index, voting_credential, coin, anchor / null)",
        )?;

        let cert_index = certificate_index_names::CertificateIndexNames::DrepRegistration.to_u64();
        deserialize_and_check_index(raw, cert_index, "cert_index")?;

        let voting_credential =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("voting_credential"))?;

        let coin = Coin::deserialize(raw).map_err(|e| e.annotate("coin"))?;

        let anchor = Anchor::deserialize_nullable(raw).map_err(|e| e.annotate("anchor"))?;

        Ok(DrepRegistration {
            voting_credential,
            coin,
            anchor,
        })
    }
}
