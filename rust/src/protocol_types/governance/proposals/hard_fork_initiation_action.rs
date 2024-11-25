use crate::*;

#[derive(
    Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
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
