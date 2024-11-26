use crate::*;

use crate::serialization::utils::serialize_and_check_index;
use crate::serialization::{check_len, deserialize_and_check_index};
use map_names::VotingProposalIndexNames;

impl Serialize for NewConstitutionAction {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;

        let proposal_index = VotingProposalIndexNames::NewConstitutionAction.to_u64();
        serialize_and_check_index(serializer, proposal_index, "NewConstitutionAction")?;

        self.gov_action_id.serialize_nullable(serializer)?;
        self.constitution.serialize(serializer)?;

        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(NewConstitutionAction);

impl DeserializeEmbeddedGroup for NewConstitutionAction {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(
            len,
            3,
            "(proposal_index, gov_action_id / null, constitution)",
        )?;

        let desired_index = VotingProposalIndexNames::NewConstitutionAction.to_u64();
        deserialize_and_check_index(raw, desired_index, "proposal_index")?;

        let gov_action_id = GovernanceActionId::deserialize_nullable(raw)
            .map_err(|e| e.annotate("gov_action_id"))?;

        let constitution =
            Constitution::deserialize(raw).map_err(|e| e.annotate("constitution"))?;

        return Ok(NewConstitutionAction {
            gov_action_id,
            constitution,
        });
    }
}
