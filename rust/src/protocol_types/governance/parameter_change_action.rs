use crate::*;
use serialization::{check_len, deserialize_and_check_index, serialize_and_check_index};
use voting_proposal_index_names::VotingProposalIndexNames;

#[derive(
    Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[wasm_bindgen]
pub struct ParameterChangeAction {
    pub(crate) gov_action_id: Option<GovernanceActionId>,
    pub(crate) protocol_param_updates: ProtocolParamUpdate,
    pub(crate) policy_hash: Option<ScriptHash>,
}

impl_to_from!(ParameterChangeAction);

#[wasm_bindgen]
impl ParameterChangeAction {
    pub fn gov_action_id(&self) -> Option<GovernanceActionId> {
        self.gov_action_id.clone()
    }

    pub fn protocol_param_updates(&self) -> ProtocolParamUpdate {
        self.protocol_param_updates.clone()
    }

    pub fn policy_hash(&self) -> Option<ScriptHash> {
        self.policy_hash.clone()
    }

    pub fn new(protocol_param_updates: &ProtocolParamUpdate) -> Self {
        Self {
            gov_action_id: None,
            protocol_param_updates: protocol_param_updates.clone(),
            policy_hash: None,
        }
    }

    pub fn new_with_action_id(
        gov_action_id: &GovernanceActionId,
        protocol_param_updates: &ProtocolParamUpdate,
    ) -> Self {
        Self {
            gov_action_id: Some(gov_action_id.clone()),
            protocol_param_updates: protocol_param_updates.clone(),
            policy_hash: None,
        }
    }

    pub fn new_with_policy_hash(
        protocol_param_updates: &ProtocolParamUpdate,
        policy_hash: &ScriptHash,
    ) -> Self {
        Self {
            gov_action_id: None,
            protocol_param_updates: protocol_param_updates.clone(),
            policy_hash: Some(policy_hash.clone()),
        }
    }

    pub fn new_with_policy_hash_and_action_id(
        gov_action_id: &GovernanceActionId,
        protocol_param_updates: &ProtocolParamUpdate,
        policy_hash: &ScriptHash,
    ) -> Self {
        Self {
            gov_action_id: Some(gov_action_id.clone()),
            protocol_param_updates: protocol_param_updates.clone(),
            policy_hash: Some(policy_hash.clone()),
        }
    }

    pub(crate) fn has_script_hash(&self) -> bool {
        self.policy_hash.is_some()
    }
}

impl cbor_event::se::Serialize for ParameterChangeAction {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;

        let proposal_index = VotingProposalIndexNames::ParameterChangeAction.to_u64();
        serialize_and_check_index(serializer, proposal_index, "ParameterChangeAction")?;

        self.gov_action_id.serialize_nullable(serializer)?;
        self.protocol_param_updates.serialize(serializer)?;
        self.policy_hash.serialize_nullable(serializer)?;

        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(ParameterChangeAction);

impl DeserializeEmbeddedGroup for ParameterChangeAction {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        let has_policy_hash = len == cbor_event::Len::Len(4) || len == cbor_event::Len::Indefinite;

        //for sancho backwards compatibility
        if !has_policy_hash {
            check_len(
                len,
                3,
                "(proposal_index, gov_action_id // null, protocol_param_updates)",
            )?;
        } else {
            check_len(
                len,
                4,
                "(proposal_index, gov_action_id // null, protocol_param_updates, policy_hash // null)",
            )?;
        }

        let desired_index = VotingProposalIndexNames::ParameterChangeAction.to_u64();
        deserialize_and_check_index(raw, desired_index, "proposal_index")?;

        let gov_action_id = GovernanceActionId::deserialize_nullable(raw)
            .map_err(|e| e.annotate("gov_action_id"))?;

        let protocol_param_updates = ProtocolParamUpdate::deserialize(raw)
            .map_err(|e| e.annotate("protocol_param_updates"))?;

        let policy_hash = if has_policy_hash {
            ScriptHash::deserialize_nullable(raw).map_err(|e| e.annotate("policy_hash"))?
        } else {
            None
        };

        return Ok(ParameterChangeAction {
            gov_action_id,
            protocol_param_updates,
            policy_hash,
        });
    }
}
