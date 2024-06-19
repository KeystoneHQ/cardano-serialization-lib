use crate::*;
use serialization::{check_len, serialize_and_check_index, deserialize_and_check_index};
use voting_proposal_index_names::VotingProposalIndexNames;

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
pub struct HardForkInitiationAction {
    pub(crate) gov_action_id: Option<GovernanceActionId>,
    pub(crate) protocol_version: ProtocolVersion,
}

impl_to_from!(HardForkInitiationAction);

#[wasm_bindgen]
impl HardForkInitiationAction {
    pub fn gov_action_id(&self) -> Option<GovernanceActionId> {
        self.gov_action_id.clone()
    }

    pub fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version.clone()
    }

    pub fn new(protocol_version: &ProtocolVersion) -> Self {
        Self {
            gov_action_id: None,
            protocol_version: protocol_version.clone(),
        }
    }

    pub fn new_with_action_id(
        gov_action_id: &GovernanceActionId,
        protocol_version: &ProtocolVersion,
    ) -> Self {
        Self {
            gov_action_id: Some(gov_action_id.clone()),
            protocol_version: protocol_version.clone(),
        }
    }
}


impl cbor_event::se::Serialize for HardForkInitiationAction {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;

        let proposal_index = VotingProposalIndexNames::HardForkInitiationAction.to_u64();
        serialize_and_check_index(serializer, proposal_index, "HardForkInitiationAction")?;

        if let Some(gov_id) = &self.gov_action_id {
            gov_id.serialize(serializer)?;
        } else {
            serializer.write_special(CBORSpecial::Null)?;
        }

        self.protocol_version.serialize(serializer)?;

        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(HardForkInitiationAction);

impl DeserializeEmbeddedGroup for HardForkInitiationAction {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(
            len,
            3,
            "(proposal_index, gov_action_id // null, [protocol_version])",
        )?;

        let desired_index = VotingProposalIndexNames::HardForkInitiationAction.to_u64();
        deserialize_and_check_index(raw, desired_index, "proposal_index")?;

        let gov_action_id = GovernanceActionId::deserialize_nullable(raw)
            .map_err(|e| e.annotate("gov_action_id"))?;

        let protocol_version = ProtocolVersion::deserialize(raw)?;

        return Ok(HardForkInitiationAction {
            gov_action_id,
            protocol_version,
        });
    }
}