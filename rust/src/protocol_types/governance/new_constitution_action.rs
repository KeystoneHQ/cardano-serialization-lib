use crate::*;
use serialization::{check_len, deserialize_and_check_index, serialize_and_check_index};
use voting_proposal_index_names::VotingProposalIndexNames;

#[derive(
    Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[wasm_bindgen]
pub struct NewConstitutionAction {
    pub(crate) gov_action_id: Option<GovernanceActionId>,
    pub(crate) constitution: Constitution,
}

impl_to_from!(NewConstitutionAction);

#[wasm_bindgen]
impl NewConstitutionAction {
    pub fn gov_action_id(&self) -> Option<GovernanceActionId> {
        self.gov_action_id.clone()
    }

    pub fn constitution(&self) -> Constitution {
        self.constitution.clone()
    }

    pub fn new(constitution: &Constitution) -> Self {
        Self {
            gov_action_id: None,
            constitution: constitution.clone(),
        }
    }

    pub fn new_with_action_id(
        gov_action_id: &GovernanceActionId,
        constitution: &Constitution,
    ) -> Self {
        Self {
            gov_action_id: Some(gov_action_id.clone()),
            constitution: constitution.clone(),
        }
    }

    pub fn has_script_hash(&self) -> bool {
        self.constitution.script_hash.is_some()
    }
}

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
