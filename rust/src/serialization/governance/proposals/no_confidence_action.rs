use crate::serialization::utils::serialize_and_check_index;
use crate::serialization::{check_len, deserialize_and_check_index};
use crate::*;
use map_names::VotingProposalIndexNames;

impl cbor_event::se::Serialize for NoConfidenceAction {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;

        let proposal_index = VotingProposalIndexNames::NoConfidenceAction.to_u64();
        serialize_and_check_index(serializer, proposal_index, "NoConfidenceAction")?;

        self.gov_action_id.serialize_nullable(serializer)?;

        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(NoConfidenceAction);

impl DeserializeEmbeddedGroup for NoConfidenceAction {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 2, "(proposal_index, gov_action_id // null)")?;

        let desired_index = VotingProposalIndexNames::NoConfidenceAction.to_u64();
        deserialize_and_check_index(raw, desired_index, "proposal_index")?;

        let gov_action_id = GovernanceActionId::deserialize_nullable(raw)
            .map_err(|e| e.annotate("gov_action_id"))?;

        return Ok(NoConfidenceAction { gov_action_id });
    }
}
