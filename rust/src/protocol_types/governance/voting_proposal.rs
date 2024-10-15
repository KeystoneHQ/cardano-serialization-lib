use crate::*;
use serialization::check_len;

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
pub struct VotingProposal {
    pub(crate) governance_action: GovernanceAction,
    pub(crate) anchor: Anchor,
    pub(crate) reward_account: RewardAddress,
    pub(crate) deposit: Coin,
}

impl_to_from!(VotingProposal);

#[wasm_bindgen]
impl VotingProposal {
    pub fn governance_action(&self) -> GovernanceAction {
        self.governance_action.clone()
    }

    pub fn anchor(&self) -> Anchor {
        self.anchor.clone()
    }

    pub fn reward_account(&self) -> RewardAddress {
        self.reward_account.clone()
    }

    pub fn deposit(&self) -> Coin {
        self.deposit.clone()
    }

    pub fn new(
        governance_action: &GovernanceAction,
        anchor: &Anchor,
        reward_account: &RewardAddress,
        deposit: &Coin,
    ) -> Self {
        Self {
            governance_action: governance_action.clone(),
            anchor: anchor.clone(),
            reward_account: reward_account.clone(),
            deposit: deposit.clone(),
        }
    }

    pub(crate) fn has_script_hash(&self) -> bool {
        match self.governance_action.0 {
            GovernanceActionEnum::ParameterChangeAction(ref action) => action.has_script_hash(),
            GovernanceActionEnum::TreasuryWithdrawalsAction(ref action) => action.has_script_hash(),
            _ => false,
        }
    }
}


impl Serialize for VotingProposal {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.deposit.serialize(serializer)?;
        self.reward_account.serialize(serializer)?;
        self.governance_action.serialize(serializer)?;
        self.anchor.serialize(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(VotingProposal);

impl DeserializeEmbeddedGroup for VotingProposal {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(
            len,
            4,
            "(deposit, reward_account, gov_action, anchor)",
        )?;

        let deposit = Coin::deserialize(raw)
            .map_err(|e| e.annotate("deposit"))?;
        let reward_account = RewardAddress::deserialize(raw)
            .map_err(|e| e.annotate("reward_account"))?;
        let gov_action = GovernanceAction::deserialize(raw)
            .map_err(|e| e.annotate("gov_action"))?;
        let anchor = Anchor::deserialize(raw)
            .map_err(|e| e.annotate("anchor"))?;

        return Ok(VotingProposal {
            deposit,
            reward_account,
            governance_action: gov_action,
            anchor,
        });
    }
}
