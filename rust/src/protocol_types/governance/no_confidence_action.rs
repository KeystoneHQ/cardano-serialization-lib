use crate::*;
use serialization::{check_len, deserialize_and_check_index, serialize_and_check_index};
use voting_proposal_index_names::VotingProposalIndexNames;

#[derive(
    Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[wasm_bindgen]
pub struct NoConfidenceAction {
    pub(crate) gov_action_id: Option<GovernanceActionId>,
}

impl_to_from!(NoConfidenceAction);

#[wasm_bindgen]
impl NoConfidenceAction {
    pub fn gov_action_id(&self) -> Option<GovernanceActionId> {
        self.gov_action_id.clone()
    }

    pub fn new() -> Self {
        Self {
            gov_action_id: None,
        }
    }

    pub fn new_with_action_id(gov_action_id: &GovernanceActionId) -> Self {
        Self {
            gov_action_id: Some(gov_action_id.clone()),
        }
    }
}

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
