use crate::*;
use crate::serialization::{
    check_len, deserialize_and_check_index, serialize_and_check_index,
};
use crate::certificate_index_names::CertificateIndexNames;

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
pub struct StakeRegistrationAndDelegation {
    pub(crate) stake_credential: StakeCredential,
    pub(crate) pool_keyhash: Ed25519KeyHash,
    pub(crate) coin: Coin,
}

impl_to_from!(StakeRegistrationAndDelegation);

#[wasm_bindgen]
impl StakeRegistrationAndDelegation {
    pub fn stake_credential(&self) -> StakeCredential {
        self.stake_credential.clone()
    }

    pub fn pool_keyhash(&self) -> Ed25519KeyHash {
        self.pool_keyhash.clone()
    }

    pub fn coin(&self) -> Coin {
        self.coin.clone()
    }

    pub fn new(stake_credential: &StakeCredential, pool_keyhash: &Ed25519KeyHash, coin: &Coin) -> Self {
        Self {
            stake_credential: stake_credential.clone(),
            pool_keyhash: pool_keyhash.clone(),
            coin: coin.clone(),
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        self.stake_credential.has_script_hash()
    }
}

impl cbor_event::se::Serialize for StakeRegistrationAndDelegation {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;

        let proposal_index = CertificateIndexNames::StakeRegistrationAndDelegation.to_u64();
        serialize_and_check_index(serializer, proposal_index, "StakeRegistrationAndDelegation")?;

        self.stake_credential.serialize(serializer)?;
        self.pool_keyhash.serialize(serializer)?;
        self.coin.serialize(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(StakeRegistrationAndDelegation);

impl DeserializeEmbeddedGroup for StakeRegistrationAndDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 4, "(cert_index, stake_credential, pool_keyhash, coin)")?;
        let cert_index = CertificateIndexNames::StakeRegistrationAndDelegation.to_u64();
        deserialize_and_check_index(raw, cert_index, "cert_index")?;

        let stake_credential =
            StakeCredential::deserialize(raw).map_err(|e| e.annotate("stake_credential"))?;

        let pool_keyhash =
            Ed25519KeyHash::deserialize(raw).map_err(|e| e.annotate("pool_keyhash"))?;

        let coin = Coin::deserialize(raw).map_err(|e| e.annotate("coin"))?;

        Ok(StakeRegistrationAndDelegation {
            stake_credential,
            pool_keyhash,
            coin,
        })
    }
}
