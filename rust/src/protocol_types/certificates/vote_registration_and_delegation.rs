use crate::certificate_index_names::CertificateIndexNames;
use crate::serialization::{check_len, deserialize_and_check_index, serialize_and_check_index};
use crate::*;

#[derive(
    Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[wasm_bindgen]
pub struct VoteRegistrationAndDelegation {
    pub(crate) stake_credential: StakeCredential,
    pub(crate) drep: DRep,
    pub(crate) coin: Coin,
}

impl_to_from!(VoteRegistrationAndDelegation);

#[wasm_bindgen]
impl VoteRegistrationAndDelegation {
    pub fn stake_credential(&self) -> StakeCredential {
        self.stake_credential.clone()
    }

    pub fn drep(&self) -> DRep {
        self.drep.clone()
    }

    pub fn coin(&self) -> Coin {
        self.coin.clone()
    }

    pub fn new(stake_credential: &StakeCredential, drep: &DRep, coin: &Coin) -> Self {
        Self {
            stake_credential: stake_credential.clone(),
            drep: drep.clone(),
            coin: coin.clone(),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.stake_credential.has_script_hash()
    }
}

impl cbor_event::se::Serialize for VoteRegistrationAndDelegation {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;

        let proposal_index = CertificateIndexNames::VoteRegistrationAndDelegation.to_u64();
        serialize_and_check_index(serializer, proposal_index, "VoteRegistrationAndDelegation")?;

        self.stake_credential.serialize(serializer)?;
        self.drep.serialize(serializer)?;
        self.coin.serialize(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(VoteRegistrationAndDelegation);

impl DeserializeEmbeddedGroup for VoteRegistrationAndDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 4, "(cert_index, stake_credential, drep, coin)")?;

        let desired_index = CertificateIndexNames::VoteRegistrationAndDelegation.to_u64();
        deserialize_and_check_index(raw, desired_index, "cert_index")?;

        let stake_credential =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("stake_credential"))?;

        let drep = DRep::deserialize(raw).map_err(|e| e.annotate("drep"))?;

        let coin = Coin::deserialize(raw).map_err(|e| e.annotate("coin"))?;

        Ok(VoteRegistrationAndDelegation {
            stake_credential,
            drep,
            coin,
        })
    }
}
