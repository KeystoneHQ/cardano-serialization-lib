use crate::serialization::map_names::CertificateIndexNames;
use crate::serialization::utils::{
    check_len, deserialize_and_check_index, serialize_and_check_index,
};
use crate::*;

impl cbor_event::se::Serialize for StakeAndVoteDelegation {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;

        let proposal_index = CertificateIndexNames::StakeAndVoteDelegation.to_u64();
        serialize_and_check_index(serializer, proposal_index, "StakeAndVoteDelegation")?;

        self.stake_credential.serialize(serializer)?;
        self.pool_keyhash.serialize(serializer)?;
        self.drep.serialize(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(StakeAndVoteDelegation);

impl DeserializeEmbeddedGroup for StakeAndVoteDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 4, "(cert_index, stake_credential, pool_keyhash, drep)")?;
        let cert_index = CertificateIndexNames::StakeAndVoteDelegation.to_u64();
        deserialize_and_check_index(raw, cert_index, "cert_index")?;

        let stake_credential =
            Credential::deserialize(raw).map_err(|e| e.annotate("stake_credential"))?;

        let pool_keyhash =
            Ed25519KeyHash::deserialize(raw).map_err(|e| e.annotate("pool_keyhash"))?;

        let drep = DRep::deserialize(raw).map_err(|e| e.annotate("drep"))?;

        Ok(StakeAndVoteDelegation {
            stake_credential,
            pool_keyhash,
            drep,
        })
    }
}
