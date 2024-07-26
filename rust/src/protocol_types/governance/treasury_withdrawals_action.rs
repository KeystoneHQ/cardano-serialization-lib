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
pub struct TreasuryWithdrawalsAction {
    pub(crate) withdrawals: TreasuryWithdrawals,
    pub(crate) policy_hash: Option<ScriptHash>,
}

impl_to_from!(TreasuryWithdrawalsAction);

#[wasm_bindgen]
impl TreasuryWithdrawalsAction {
    pub fn withdrawals(&self) -> TreasuryWithdrawals {
        self.withdrawals.clone()
    }

    pub fn policy_hash(&self) -> Option<ScriptHash> {
        self.policy_hash.clone()
    }

    pub fn new(withdrawals: &TreasuryWithdrawals) -> Self {
        Self {
            withdrawals: withdrawals.clone(),
            policy_hash: None,
        }
    }

    pub fn new_with_policy_hash(
        withdrawals: &TreasuryWithdrawals,
        policy_hash: &ScriptHash,
    ) -> Self {
        Self {
            withdrawals: withdrawals.clone(),
            policy_hash: Some(policy_hash.clone()),
        }
    }

    pub(crate) fn has_script_hash(&self) -> bool {
        self.policy_hash.is_some()
    }
}


impl Serialize for TreasuryWithdrawalsAction {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;

        let proposal_index = VotingProposalIndexNames::TreasuryWithdrawalsAction.to_u64();
        serialize_and_check_index(serializer, proposal_index, "TreasuryWithdrawalsAction")?;

        self.withdrawals.serialize(serializer)?;
        self.policy_hash.serialize_nullable(serializer)?;

        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(TreasuryWithdrawalsAction);

impl DeserializeEmbeddedGroup for TreasuryWithdrawalsAction {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {

        let has_policy_hash = len == cbor_event::Len::Len(3) || len == cbor_event::Len::Indefinite;

        //for sancho backwards compatibility
        if !has_policy_hash {
            check_len(len, 2, "(proposal_index, { reward_account => coin })")?;
        } else {
            check_len(len, 3, "(proposal_index, { reward_account => coin }, policy_hash / null)")?;
        }


        let desired_index = VotingProposalIndexNames::TreasuryWithdrawalsAction.to_u64();
        deserialize_and_check_index(raw, desired_index, "proposal_index")?;

        let withdrawals = TreasuryWithdrawals::deserialize(raw)?;

        let policy_hash = if has_policy_hash {
            ScriptHash::deserialize_nullable(raw)?
        } else {
            None
        };

        return Ok(TreasuryWithdrawalsAction { withdrawals , policy_hash});
    }
}