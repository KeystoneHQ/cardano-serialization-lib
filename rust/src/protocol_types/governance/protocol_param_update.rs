use crate::*;
use serialization::check_len;

#[wasm_bindgen]
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
pub struct PoolVotingThresholds {
    pub(crate) motion_no_confidence: UnitInterval,
    pub(crate) committee_normal: UnitInterval,
    pub(crate) committee_no_confidence: UnitInterval,
    pub(crate) hard_fork_initiation: UnitInterval,
    pub(crate) security_relevant_threshold: UnitInterval,
}

impl_to_from!(PoolVotingThresholds);

#[wasm_bindgen]
impl PoolVotingThresholds {
    pub fn new(
        motion_no_confidence: &UnitInterval,
        committee_normal: &UnitInterval,
        committee_no_confidence: &UnitInterval,
        hard_fork_initiation: &UnitInterval,
        security_relevant_threshold: &UnitInterval,
    ) -> Self {
        Self {
            motion_no_confidence: motion_no_confidence.clone(),
            committee_normal: committee_normal.clone(),
            committee_no_confidence: committee_no_confidence.clone(),
            hard_fork_initiation: hard_fork_initiation.clone(),
            security_relevant_threshold: security_relevant_threshold.clone(),
        }
    }

    pub fn motion_no_confidence(&self) -> UnitInterval {
        self.motion_no_confidence.clone()
    }

    pub fn committee_normal(&self) -> UnitInterval {
        self.committee_normal.clone()
    }

    pub fn committee_no_confidence(&self) -> UnitInterval {
        self.committee_no_confidence.clone()
    }

    pub fn hard_fork_initiation(&self) -> UnitInterval {
        self.hard_fork_initiation.clone()
    }
}

#[wasm_bindgen]
#[derive(
    Clone,
    Debug,
    Hash,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct DrepVotingThresholds {
    pub(crate) motion_no_confidence: UnitInterval,
    pub(crate) committee_normal: UnitInterval,
    pub(crate) committee_no_confidence: UnitInterval,
    pub(crate) update_constitution: UnitInterval,
    pub(crate) hard_fork_initiation: UnitInterval,
    pub(crate) pp_network_group: UnitInterval,
    pub(crate) pp_economic_group: UnitInterval,
    pub(crate) pp_technical_group: UnitInterval,
    pub(crate) pp_governance_group: UnitInterval,
    pub(crate) treasury_withdrawal: UnitInterval,
}

impl_to_from!(DrepVotingThresholds);

#[wasm_bindgen]
impl DrepVotingThresholds {
    pub fn new(
        motion_no_confidence: &UnitInterval,
        committee_normal: &UnitInterval,
        committee_no_confidence: &UnitInterval,
        update_constitution: &UnitInterval,
        hard_fork_initiation: &UnitInterval,
        pp_network_group: &UnitInterval,
        pp_economic_group: &UnitInterval,
        pp_technical_group: &UnitInterval,
        pp_governance_group: &UnitInterval,
        treasury_withdrawal: &UnitInterval,
    ) -> Self {
        Self {
            motion_no_confidence: motion_no_confidence.clone(),
            committee_normal: committee_normal.clone(),
            committee_no_confidence: committee_no_confidence.clone(),
            update_constitution: update_constitution.clone(),
            hard_fork_initiation: hard_fork_initiation.clone(),
            pp_network_group: pp_network_group.clone(),
            pp_economic_group: pp_economic_group.clone(),
            pp_technical_group: pp_technical_group.clone(),
            pp_governance_group: pp_governance_group.clone(),
            treasury_withdrawal: treasury_withdrawal.clone(),
        }
    }

    pub fn new_default() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn set_motion_no_confidence(&mut self, motion_no_confidence: &UnitInterval) {
        self.motion_no_confidence = motion_no_confidence.clone()
    }

    pub fn set_committee_normal(&mut self, committee_normal: &UnitInterval) {
        self.committee_normal = committee_normal.clone()
    }

    pub fn set_committee_no_confidence(&mut self, committee_no_confidence: &UnitInterval) {
        self.committee_no_confidence = committee_no_confidence.clone()
    }

    pub fn set_update_constitution(&mut self, update_constitution: &UnitInterval) {
        self.update_constitution = update_constitution.clone()
    }

    pub fn set_hard_fork_initiation(&mut self, hard_fork_initiation: &UnitInterval) {
        self.hard_fork_initiation = hard_fork_initiation.clone()
    }

    pub fn set_pp_network_group(&mut self, pp_network_group: &UnitInterval) {
        self.pp_network_group = pp_network_group.clone()
    }

    pub fn set_pp_economic_group(&mut self, pp_economic_group: &UnitInterval) {
        self.pp_economic_group = pp_economic_group.clone()
    }

    pub fn set_pp_technical_group(&mut self, pp_technical_group: &UnitInterval) {
        self.pp_technical_group = pp_technical_group.clone()
    }

    pub fn set_pp_governance_group(&mut self, pp_governance_group: &UnitInterval) {
        self.pp_governance_group = pp_governance_group.clone()
    }

    pub fn set_treasury_withdrawal(&mut self, treasury_withdrawal: &UnitInterval) {
        self.treasury_withdrawal = treasury_withdrawal.clone()
    }

    pub fn motion_no_confidence(&self) -> UnitInterval {
        self.motion_no_confidence.clone()
    }

    pub fn committee_normal(&self) -> UnitInterval {
        self.committee_normal.clone()
    }

    pub fn committee_no_confidence(&self) -> UnitInterval {
        self.committee_no_confidence.clone()
    }

    pub fn update_constitution(&self) -> UnitInterval {
        self.update_constitution.clone()
    }

    pub fn hard_fork_initiation(&self) -> UnitInterval {
        self.hard_fork_initiation.clone()
    }

    pub fn pp_network_group(&self) -> UnitInterval {
        self.pp_network_group.clone()
    }

    pub fn pp_economic_group(&self) -> UnitInterval {
        self.pp_economic_group.clone()
    }

    pub fn pp_technical_group(&self) -> UnitInterval {
        self.pp_technical_group.clone()
    }

    pub fn pp_governance_group(&self) -> UnitInterval {
        self.pp_governance_group.clone()
    }

    pub fn treasury_withdrawal(&self) -> UnitInterval {
        self.treasury_withdrawal.clone()
    }
}

#[wasm_bindgen]
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
pub struct ProtocolParamUpdate {
    pub(crate) minfee_a: Option<Coin>,
    pub(crate) minfee_b: Option<Coin>,
    pub(crate) max_block_body_size: Option<u32>,
    pub(crate) max_tx_size: Option<u32>,
    pub(crate) max_block_header_size: Option<u32>,
    pub(crate) key_deposit: Option<Coin>,
    pub(crate) pool_deposit: Option<Coin>,
    pub(crate) max_epoch: Option<Epoch>,
    // desired number of stake pools
    pub(crate) n_opt: Option<u32>,
    pub(crate) pool_pledge_influence: Option<UnitInterval>,
    pub(crate) expansion_rate: Option<UnitInterval>,
    pub(crate) treasury_growth_rate: Option<UnitInterval>,
    // decentralization constant
    pub(crate) d: Option<UnitInterval>,
    pub(crate) extra_entropy: Option<Nonce>,
    pub(crate) protocol_version: Option<ProtocolVersion>,
    pub(crate) min_pool_cost: Option<Coin>,
    pub(crate) ada_per_utxo_byte: Option<Coin>,
    pub(crate) cost_models: Option<Costmdls>,
    pub(crate) execution_costs: Option<ExUnitPrices>,
    pub(crate) max_tx_ex_units: Option<ExUnits>,
    pub(crate) max_block_ex_units: Option<ExUnits>,
    pub(crate) max_value_size: Option<u32>,
    pub(crate) collateral_percentage: Option<u32>,
    pub(crate) max_collateral_inputs: Option<u32>,
    pub(crate) pool_voting_thresholds: Option<PoolVotingThresholds>,
    pub(crate) drep_voting_thresholds: Option<DrepVotingThresholds>,
    pub(crate) min_committee_size: Option<u32>,
    pub(crate) committee_term_limit: Option<u32>,
    pub(crate) governance_action_validity_period: Option<Epoch>,
    pub(crate) governance_action_deposit: Option<Coin>,
    pub(crate) drep_deposit: Option<Coin>,
    pub(crate) drep_inactivity_period: Option<Epoch>,
    pub(crate) ref_script_coins_per_byte: Option<UnitInterval>,
}

impl_to_from!(ProtocolParamUpdate);

#[wasm_bindgen]
impl ProtocolParamUpdate {
    pub fn set_minfee_a(&mut self, minfee_a: &Coin) {
        self.minfee_a = Some(minfee_a.clone())
    }

    pub fn minfee_a(&self) -> Option<Coin> {
        self.minfee_a.clone()
    }

    pub fn set_minfee_b(&mut self, minfee_b: &Coin) {
        self.minfee_b = Some(minfee_b.clone())
    }

    pub fn minfee_b(&self) -> Option<Coin> {
        self.minfee_b.clone()
    }

    pub fn set_max_block_body_size(&mut self, max_block_body_size: u32) {
        self.max_block_body_size = Some(max_block_body_size)
    }

    pub fn max_block_body_size(&self) -> Option<u32> {
        self.max_block_body_size.clone()
    }

    pub fn set_max_tx_size(&mut self, max_tx_size: u32) {
        self.max_tx_size = Some(max_tx_size)
    }

    pub fn max_tx_size(&self) -> Option<u32> {
        self.max_tx_size.clone()
    }

    pub fn set_max_block_header_size(&mut self, max_block_header_size: u32) {
        self.max_block_header_size = Some(max_block_header_size)
    }

    pub fn max_block_header_size(&self) -> Option<u32> {
        self.max_block_header_size.clone()
    }

    pub fn set_key_deposit(&mut self, key_deposit: &Coin) {
        self.key_deposit = Some(key_deposit.clone())
    }

    pub fn key_deposit(&self) -> Option<Coin> {
        self.key_deposit.clone()
    }

    pub fn set_pool_deposit(&mut self, pool_deposit: &Coin) {
        self.pool_deposit = Some(pool_deposit.clone())
    }

    pub fn pool_deposit(&self) -> Option<Coin> {
        self.pool_deposit.clone()
    }

    pub fn set_max_epoch(&mut self, max_epoch: Epoch) {
        self.max_epoch = Some(max_epoch.clone())
    }

    pub fn max_epoch(&self) -> Option<Epoch> {
        self.max_epoch.clone()
    }

    pub fn set_n_opt(&mut self, n_opt: u32) {
        self.n_opt = Some(n_opt)
    }

    pub fn n_opt(&self) -> Option<u32> {
        self.n_opt.clone()
    }

    pub fn set_pool_pledge_influence(&mut self, pool_pledge_influence: &UnitInterval) {
        self.pool_pledge_influence = Some(pool_pledge_influence.clone())
    }

    pub fn pool_pledge_influence(&self) -> Option<UnitInterval> {
        self.pool_pledge_influence.clone()
    }

    pub fn set_expansion_rate(&mut self, expansion_rate: &UnitInterval) {
        self.expansion_rate = Some(expansion_rate.clone())
    }

    pub fn expansion_rate(&self) -> Option<UnitInterval> {
        self.expansion_rate.clone()
    }

    pub fn set_treasury_growth_rate(&mut self, treasury_growth_rate: &UnitInterval) {
        self.treasury_growth_rate = Some(treasury_growth_rate.clone())
    }

    pub fn treasury_growth_rate(&self) -> Option<UnitInterval> {
        self.treasury_growth_rate.clone()
    }

    /// !!! DEPRECATED !!!
    /// Since babbage era this param is outdated. But this param you can meet in a pre-babbage block.
    #[deprecated(
        since = "11.0.0",
        note = "Since babbage era this param is outdated. But this param you can meet in a pre-babbage block."
    )]
    pub fn d(&self) -> Option<UnitInterval> {
        self.d.clone()
    }

    /// !!! DEPRECATED !!!
    /// Since babbage era this param is outdated. But this param you can meet in a pre-babbage block.
    #[deprecated(
        since = "11.0.0",
        note = "Since babbage era this param is outdated. But this param you can meet in a pre-babbage block."
    )]
    pub fn extra_entropy(&self) -> Option<Nonce> {
        self.extra_entropy.clone()
    }

    /// !!! DEPRECATED !!!
    /// Since conway era this param is outdated. But this param you can meet in a pre-conway block.
    #[deprecated(
        since = "12.0.0",
        note = "Since conway era this param is outdated. But this param you can meet in a pre-conway block."
    )]
    pub fn set_protocol_version(&mut self, protocol_version: &ProtocolVersion) {
        self.protocol_version = Some(protocol_version.clone())
    }

    pub fn protocol_version(&self) -> Option<ProtocolVersion> {
        self.protocol_version.clone()
    }

    pub fn set_min_pool_cost(&mut self, min_pool_cost: &Coin) {
        self.min_pool_cost = Some(min_pool_cost.clone())
    }

    pub fn min_pool_cost(&self) -> Option<Coin> {
        self.min_pool_cost.clone()
    }

    pub fn set_ada_per_utxo_byte(&mut self, ada_per_utxo_byte: &Coin) {
        self.ada_per_utxo_byte = Some(ada_per_utxo_byte.clone())
    }

    pub fn ada_per_utxo_byte(&self) -> Option<Coin> {
        self.ada_per_utxo_byte.clone()
    }

    pub fn set_cost_models(&mut self, cost_models: &Costmdls) {
        self.cost_models = Some(cost_models.clone())
    }

    pub fn cost_models(&self) -> Option<Costmdls> {
        self.cost_models.clone()
    }

    pub fn set_execution_costs(&mut self, execution_costs: &ExUnitPrices) {
        self.execution_costs = Some(execution_costs.clone())
    }

    pub fn execution_costs(&self) -> Option<ExUnitPrices> {
        self.execution_costs.clone()
    }

    pub fn set_max_tx_ex_units(&mut self, max_tx_ex_units: &ExUnits) {
        self.max_tx_ex_units = Some(max_tx_ex_units.clone())
    }

    pub fn max_tx_ex_units(&self) -> Option<ExUnits> {
        self.max_tx_ex_units.clone()
    }

    pub fn set_max_block_ex_units(&mut self, max_block_ex_units: &ExUnits) {
        self.max_block_ex_units = Some(max_block_ex_units.clone())
    }

    pub fn max_block_ex_units(&self) -> Option<ExUnits> {
        self.max_block_ex_units.clone()
    }

    pub fn set_max_value_size(&mut self, max_value_size: u32) {
        self.max_value_size = Some(max_value_size.clone())
    }

    pub fn max_value_size(&self) -> Option<u32> {
        self.max_value_size.clone()
    }

    pub fn set_collateral_percentage(&mut self, collateral_percentage: u32) {
        self.collateral_percentage = Some(collateral_percentage)
    }

    pub fn collateral_percentage(&self) -> Option<u32> {
        self.collateral_percentage.clone()
    }

    pub fn set_max_collateral_inputs(&mut self, max_collateral_inputs: u32) {
        self.max_collateral_inputs = Some(max_collateral_inputs)
    }

    pub fn max_collateral_inputs(&self) -> Option<u32> {
        self.max_collateral_inputs.clone()
    }

    pub fn set_pool_voting_thresholds(&mut self, pool_voting_thresholds: &PoolVotingThresholds) {
        self.pool_voting_thresholds = Some(pool_voting_thresholds.clone())
    }

    pub fn pool_voting_thresholds(&self) -> Option<PoolVotingThresholds> {
        self.pool_voting_thresholds.clone()
    }

    pub fn set_drep_voting_thresholds(&mut self, drep_voting_thresholds: &DrepVotingThresholds) {
        self.drep_voting_thresholds = Some(drep_voting_thresholds.clone())
    }

    pub fn drep_voting_thresholds(&self) -> Option<DrepVotingThresholds> {
        self.drep_voting_thresholds.clone()
    }

    pub fn set_min_committee_size(&mut self, min_committee_size: u32) {
        self.min_committee_size = Some(min_committee_size)
    }

    pub fn min_committee_size(&self) -> Option<u32> {
        self.min_committee_size.clone()
    }

    pub fn set_committee_term_limit(&mut self, committee_term_limit: u32) {
        self.committee_term_limit = Some(committee_term_limit)
    }

    pub fn committee_term_limit(&self) -> Option<u32> {
        self.committee_term_limit.clone()
    }

    pub fn set_governance_action_validity_period(&mut self, governance_action_validity_period: Epoch) {
        self.governance_action_validity_period = Some(governance_action_validity_period)
    }

    pub fn governance_action_validity_period(&self) -> Option<Epoch> {
        self.governance_action_validity_period.clone()
    }

    pub fn set_governance_action_deposit(&mut self, governance_action_deposit: &Coin) {
        self.governance_action_deposit = Some(governance_action_deposit.clone());
    }

    pub fn governance_action_deposit(&self) -> Option<Coin> {
        self.governance_action_deposit.clone()
    }

    pub fn set_drep_deposit(&mut self, drep_deposit: &Coin) {
        self.drep_deposit = Some(drep_deposit.clone());
    }

    pub fn drep_deposit(&self) -> Option<Coin> {
        self.drep_deposit.clone()
    }

    pub fn set_drep_inactivity_period(&mut self, drep_inactivity_period: Epoch) {
        self.drep_inactivity_period = Some(drep_inactivity_period)
    }

    pub fn drep_inactivity_period(&self) -> Option<Epoch> {
        self.drep_inactivity_period.clone()
    }

    pub fn set_ref_script_coins_per_byte(&mut self, ref_script_coins_per_byte: &UnitInterval) {
        self.ref_script_coins_per_byte = Some(ref_script_coins_per_byte.clone());
    }

    pub fn ref_script_coins_per_byte(&self) -> Option<UnitInterval> {
        self.ref_script_coins_per_byte.clone()
    }

    pub fn new() -> Self {
        Self {
            minfee_a: None,
            minfee_b: None,
            max_block_body_size: None,
            max_tx_size: None,
            max_block_header_size: None,
            key_deposit: None,
            pool_deposit: None,
            max_epoch: None,
            n_opt: None,
            pool_pledge_influence: None,
            expansion_rate: None,
            treasury_growth_rate: None,
            d: None,
            extra_entropy: None,
            protocol_version: None,
            min_pool_cost: None,
            ada_per_utxo_byte: None,
            cost_models: None,
            execution_costs: None,
            max_tx_ex_units: None,
            max_block_ex_units: None,
            max_value_size: None,
            collateral_percentage: None,
            max_collateral_inputs: None,
            pool_voting_thresholds: None,
            drep_voting_thresholds: None,
            min_committee_size: None,
            committee_term_limit: None,
            governance_action_validity_period: None,
            governance_action_deposit: None,
            drep_deposit: None,
            drep_inactivity_period: None,
            ref_script_coins_per_byte: None,
        }
    }
}


impl Serialize for PoolVotingThresholds {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(5))?;
        self.motion_no_confidence.serialize(serializer)?;
        self.committee_normal.serialize(serializer)?;
        self.committee_no_confidence.serialize(serializer)?;
        self.hard_fork_initiation.serialize(serializer)?;
        self.security_relevant_threshold.serialize(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(PoolVotingThresholds);

impl DeserializeEmbeddedGroup for PoolVotingThresholds {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(
            len,
            5,
            "[\
            motion_no_confidence, \
            committee_normal, \
            committee_no_confidence, \
            hard_fork_initiation, \
            security_relevant_threshold\
            ]",
        )?;

        let motion_no_confidence =
            UnitInterval::deserialize(raw).map_err(|e| e.annotate("motion_no_confidence"))?;
        let committee_normal =
            UnitInterval::deserialize(raw).map_err(|e| e.annotate("committee_normal"))?;
        let committee_no_confidence =
            UnitInterval::deserialize(raw).map_err(|e| e.annotate("committee_no_confidence"))?;
        let hard_fork_initiation =
            UnitInterval::deserialize(raw).map_err(|e| e.annotate("hard_fork_initiation"))?;
        let security_relevant_threshold = UnitInterval::deserialize(raw)
            .map_err(|e| e.annotate("security_relevant_threshold"))?;

        return Ok(PoolVotingThresholds {
            motion_no_confidence,
            committee_normal,
            committee_no_confidence,
            hard_fork_initiation,
            security_relevant_threshold,
        });
    }
}

impl Serialize for DrepVotingThresholds {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(10))?;
        self.motion_no_confidence.serialize(serializer)?;
        self.committee_normal.serialize(serializer)?;
        self.committee_no_confidence.serialize(serializer)?;
        self.update_constitution.serialize(serializer)?;
        self.hard_fork_initiation.serialize(serializer)?;
        self.pp_network_group.serialize(serializer)?;
        self.pp_economic_group.serialize(serializer)?;
        self.pp_technical_group.serialize(serializer)?;
        self.pp_governance_group.serialize(serializer)?;
        self.treasury_withdrawal.serialize(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(DrepVotingThresholds);

impl DeserializeEmbeddedGroup for DrepVotingThresholds {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(
            len,
            10,
            "[\
            motion_no_confidence, \
            committee_normal, \
            committee_no_confidence, \
            update_constitution, \
            hard_fork_initiation, \
            pp_network_group, \
            pp_economic_group, \
            pp_technical_group, \
            pp_governance_group, \
            treasury_withdrawal\
            ]",
        )?;

        let motion_no_confidence =
            UnitInterval::deserialize(raw).map_err(|e| e.annotate("motion_no_confidence"))?;
        let committee_normal =
            UnitInterval::deserialize(raw).map_err(|e| e.annotate("committee_normal"))?;
        let committee_no_confidence =
            UnitInterval::deserialize(raw).map_err(|e| e.annotate("committee_no_confidence"))?;
        let update_constitution =
            UnitInterval::deserialize(raw).map_err(|e| e.annotate("update_constitution"))?;
        let hard_fork_initiation =
            UnitInterval::deserialize(raw).map_err(|e| e.annotate("hard_fork_initiation"))?;
        let pp_network_group =
            UnitInterval::deserialize(raw).map_err(|e| e.annotate("pp_network_group"))?;
        let pp_economic_group =
            UnitInterval::deserialize(raw).map_err(|e| e.annotate("pp_economic_group"))?;
        let pp_technical_group =
            UnitInterval::deserialize(raw).map_err(|e| e.annotate("pp_technical_group"))?;
        let pp_governance_group =
            UnitInterval::deserialize(raw).map_err(|e| e.annotate("pp_governance_group"))?;
        let treasury_withdrawal =
            UnitInterval::deserialize(raw).map_err(|e| e.annotate("treasury_withdrawal"))?;

        return Ok(DrepVotingThresholds {
            motion_no_confidence,
            committee_normal,
            committee_no_confidence,
            update_constitution,
            hard_fork_initiation,
            pp_network_group,
            pp_economic_group,
            pp_technical_group,
            pp_governance_group,
            treasury_withdrawal,
        });
    }
}

impl cbor_event::se::Serialize for ProtocolParamUpdate {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(
            match &self.minfee_a {
                Some(_) => 1,
                None => 0,
            } + match &self.minfee_b {
                Some(_) => 1,
                None => 0,
            } + match &self.max_block_body_size {
                Some(_) => 1,
                None => 0,
            } + match &self.max_tx_size {
                Some(_) => 1,
                None => 0,
            } + match &self.max_block_header_size {
                Some(_) => 1,
                None => 0,
            } + match &self.key_deposit {
                Some(_) => 1,
                None => 0,
            } + match &self.pool_deposit {
                Some(_) => 1,
                None => 0,
            } + match &self.max_epoch {
                Some(_) => 1,
                None => 0,
            } + match &self.n_opt {
                Some(_) => 1,
                None => 0,
            } + match &self.pool_pledge_influence {
                Some(_) => 1,
                None => 0,
            } + match &self.expansion_rate {
                Some(_) => 1,
                None => 0,
            } + match &self.treasury_growth_rate {
                Some(_) => 1,
                None => 0,
            } + match &self.d {
                Some(_) => 1,
                None => 0,
            } + match &self.extra_entropy {
                Some(_) => 1,
                None => 0,
            } + match &self.protocol_version {
                Some(_) => 1,
                None => 0,
            } + match &self.min_pool_cost {
                Some(_) => 1,
                None => 0,
            } + match &self.ada_per_utxo_byte {
                Some(_) => 1,
                None => 0,
            } + match &self.cost_models {
                Some(_) => 1,
                None => 0,
            } + match &self.execution_costs {
                Some(_) => 1,
                None => 0,
            } + match &self.max_tx_ex_units {
                Some(_) => 1,
                None => 0,
            } + match &self.max_block_ex_units {
                Some(_) => 1,
                None => 0,
            } + match &self.max_value_size {
                Some(_) => 1,
                None => 0,
            } + match &self.collateral_percentage {
                Some(_) => 1,
                None => 0,
            } + match &self.max_collateral_inputs {
                Some(_) => 1,
                None => 0,
            } + match &self.pool_voting_thresholds {
                Some(_) => 1,
                None => 0,
            } + match &self.drep_voting_thresholds {
                Some(_) => 1,
                None => 0,
            } + match &self.min_committee_size {
                Some(_) => 1,
                None => 0,
            } + match &self.committee_term_limit {
                Some(_) => 1,
                None => 0,
            } + match &self.governance_action_validity_period {
                Some(_) => 1,
                None => 0,
            } + match &self.governance_action_deposit {
                Some(_) => 1,
                None => 0,
            } + match &self.drep_deposit {
                Some(_) => 1,
                None => 0,
            } + match &self.drep_inactivity_period {
                Some(_) => 1,
                None => 0,
            } + match &self.ref_script_coins_per_byte {
                Some(_) => 1,
                None => 0,
            }
        ))?;
        if let Some(field) = &self.minfee_a {
            serializer.write_unsigned_integer(0)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.minfee_b {
            serializer.write_unsigned_integer(1)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_block_body_size {
            serializer.write_unsigned_integer(2)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_tx_size {
            serializer.write_unsigned_integer(3)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_block_header_size {
            serializer.write_unsigned_integer(4)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.key_deposit {
            serializer.write_unsigned_integer(5)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.pool_deposit {
            serializer.write_unsigned_integer(6)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_epoch {
            serializer.write_unsigned_integer(7)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.n_opt {
            serializer.write_unsigned_integer(8)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.pool_pledge_influence {
            serializer.write_unsigned_integer(9)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.expansion_rate {
            serializer.write_unsigned_integer(10)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.treasury_growth_rate {
            serializer.write_unsigned_integer(11)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.d {
            serializer.write_unsigned_integer(12)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.extra_entropy {
            serializer.write_unsigned_integer(13)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.protocol_version {
            serializer.write_unsigned_integer(14)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.min_pool_cost {
            serializer.write_unsigned_integer(16)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.ada_per_utxo_byte {
            serializer.write_unsigned_integer(17)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.cost_models {
            serializer.write_unsigned_integer(18)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.execution_costs {
            serializer.write_unsigned_integer(19)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_tx_ex_units {
            serializer.write_unsigned_integer(20)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_block_ex_units {
            serializer.write_unsigned_integer(21)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_value_size {
            serializer.write_unsigned_integer(22)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.collateral_percentage {
            serializer.write_unsigned_integer(23)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.max_collateral_inputs {
            serializer.write_unsigned_integer(24)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.pool_voting_thresholds {
            serializer.write_unsigned_integer(25)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.drep_voting_thresholds {
            serializer.write_unsigned_integer(26)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.min_committee_size {
            serializer.write_unsigned_integer(27)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.committee_term_limit {
            serializer.write_unsigned_integer(28)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.governance_action_validity_period {
            serializer.write_unsigned_integer(29)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.governance_action_deposit {
            serializer.write_unsigned_integer(30)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.drep_deposit {
            serializer.write_unsigned_integer(31)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.drep_inactivity_period {
            serializer.write_unsigned_integer(32)?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.ref_script_coins_per_byte {
            serializer.write_unsigned_integer(33)?;
            field.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for ProtocolParamUpdate {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut read_len = CBORReadLen::new(len);
            let mut minfee_a = None;
            let mut minfee_b = None;
            let mut max_block_body_size = None;
            let mut max_tx_size = None;
            let mut max_block_header_size = None;
            let mut key_deposit = None;
            let mut pool_deposit = None;
            let mut max_epoch = None;
            let mut n_opt = None;
            let mut pool_pledge_influence = None;
            let mut expansion_rate = None;
            let mut treasury_growth_rate = None;
            let mut d = None;
            let mut extra_entropy = None;
            let mut protocol_version = None;
            let mut min_pool_cost = None;
            let mut ada_per_utxo_byte = None;
            let mut cost_models = None;
            let mut execution_costs = None;
            let mut max_tx_ex_units = None;
            let mut max_block_ex_units = None;
            let mut max_value_size = None;
            let mut collateral_percentage = None;
            let mut max_collateral_inputs = None;
            let mut pool_voting_thresholds = None;
            let mut drep_voting_thresholds = None;
            let mut min_committee_size = None;
            let mut committee_term_limit = None;
            let mut governance_action_validity_period = None;
            let mut governance_action_deposit = None;
            let mut drep_deposit = None;
            let mut drep_inactivity_period = None;
            let mut ref_script_coins_per_byte = None;

            let mut read = 0;
            while match len {
                cbor_event::Len::Len(n) => read < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                        0 => {
                            if minfee_a.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            minfee_a = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("minfee_a"))?,
                            );
                        }
                        1 => {
                            if minfee_b.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            minfee_b = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("minfee_b"))?,
                            );
                        }
                        2 => {
                            if max_block_body_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            max_block_body_size = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("max_block_body_size"))?,
                            );
                        }
                        3 => {
                            if max_tx_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            max_tx_size = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("max_tx_size"))?,
                            );
                        }
                        4 => {
                            if max_block_header_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                            }
                            max_block_header_size = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("max_block_header_size"))?,
                            );
                        }
                        5 => {
                            if key_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                            }
                            key_deposit = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("key_deposit"))?,
                            );
                        }
                        6 => {
                            if pool_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                            }
                            pool_deposit = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("pool_deposit"))?,
                            );
                        }
                        7 => {
                            if max_epoch.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                            }
                            max_epoch = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Epoch::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("max_epoch"))?,
                            );
                        }
                        8 => {
                            if n_opt.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(8)).into());
                            }
                            n_opt = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("n_opt"))?,
                            );
                        }
                        9 => {
                            if pool_pledge_influence.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(9)).into());
                            }
                            pool_pledge_influence = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(UnitInterval::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("pool_pledge_influence"))?,
                            );
                        }
                        10 => {
                            if expansion_rate.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(10)).into());
                            }
                            expansion_rate = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(UnitInterval::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("expansion_rate"))?,
                            );
                        }
                        11 => {
                            if treasury_growth_rate.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(11)).into());
                            }
                            treasury_growth_rate = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(UnitInterval::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("treasury_growth_rate"))?,
                            );
                        }
                        12 => {
                            if d.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(12)).into());
                            }
                            d = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(UnitInterval::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("d"))?,
                            );
                        }
                        13 => {
                            if extra_entropy.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(13)).into());
                            }
                            extra_entropy = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Nonce::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("extra_entropy"))?,
                            );
                        }
                        14 => {
                            if protocol_version.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(14)).into());
                            }
                            protocol_version = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(ProtocolVersion::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("protocol_version"))?,
                            );
                        }
                        16 => {
                            if min_pool_cost.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(16)).into());
                            }
                            min_pool_cost = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("min_pool_cost"))?,
                            );
                        }
                        17 => {
                            if ada_per_utxo_byte.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(17)).into());
                            }
                            ada_per_utxo_byte = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("ada_per_utxo_byte"))?,
                            );
                        }
                        18 => {
                            if cost_models.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(18)).into());
                            }
                            cost_models = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Costmdls::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("cost_models"))?,
                            );
                        }
                        19 => {
                            if execution_costs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(19)).into());
                            }
                            execution_costs = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(ExUnitPrices::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("execution_costs"))?,
                            );
                        }
                        20 => {
                            if max_tx_ex_units.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(20)).into());
                            }
                            max_tx_ex_units = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(ExUnits::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("max_tx_ex_units"))?,
                            );
                        }
                        21 => {
                            if max_block_ex_units.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(21)).into());
                            }
                            max_block_ex_units = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(ExUnits::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("max_block_ex_units"))?,
                            );
                        }
                        22 => {
                            if max_value_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(22)).into());
                            }
                            max_value_size = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("max_value_size"))?,
                            );
                        }
                        23 => {
                            if collateral_percentage.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(23)).into());
                            }
                            collateral_percentage = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("collateral_percentage"))?,
                            );
                        }
                        24 => {
                            if max_collateral_inputs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(24)).into());
                            }
                            max_collateral_inputs = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("max_collateral_inputs"))?,
                            );
                        }
                        25 => {
                            if pool_voting_thresholds.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(25)).into());
                            }
                            pool_voting_thresholds = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(PoolVotingThresholds::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("pool_voting_thresholds"))?,
                            );
                        }
                        26 => {
                            if drep_voting_thresholds.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(26)).into());
                            }
                            drep_voting_thresholds = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(DrepVotingThresholds::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("drep_voting_thresholds"))?,
                            );
                        }
                        27 => {
                            if min_committee_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(27)).into());
                            }
                            min_committee_size = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(u32::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("min_committee_size"))?,
                            );
                        }
                        28 => {
                            if committee_term_limit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(28)).into());
                            }
                            committee_term_limit = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Epoch::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("committee_term_limit"))?,
                            );
                        }
                        29 => {
                            if governance_action_validity_period.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(29)).into());
                            }
                            governance_action_validity_period = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Epoch::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("governance_action_validity_period"))?,
                            );
                        }
                        30 => {
                            if governance_action_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(30)).into());
                            }
                            governance_action_deposit = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("governance_action_deposit"))?,
                            );
                        }
                        31 => {
                            if drep_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(31)).into());
                            }
                            drep_deposit = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Coin::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("drep_deposit"))?,
                            );
                        }
                        32 => {
                            if drep_inactivity_period.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(32)).into());
                            }
                            drep_inactivity_period = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(Epoch::deserialize(raw)?)
                                })()
                                .map_err(|e| e.annotate("drep_inactivity_period"))?,
                            );
                        }
                        33 => {
                            if ref_script_coins_per_byte.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(33)).into());
                            }
                            ref_script_coins_per_byte = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    Ok(UnitInterval::deserialize(raw)?)
                                })()
                                    .map_err(|e| e.annotate("ref_script_coins_per_byte"))?,
                            );
                        }
                        unknown_key => {
                            return Err(
                                DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()
                            )
                        }
                    },
                    CBORType::Text => match raw.text()?.as_str() {
                        unknown_key => {
                            return Err(DeserializeFailure::UnknownKey(Key::Str(
                                "unknown_key".to_string(),
                            ))
                            .into())
                        }
                    },
                    CBORType::Special => match len {
                        cbor_event::Len::Len(_) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into())
                        }
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => {
                        return Err(DeserializeFailure::UnexpectedKeyType(other_type).into())
                    }
                }
                read += 1;
            }
            read_len.finish()?;
            Ok(Self {
                minfee_a,
                minfee_b,
                max_block_body_size,
                max_tx_size,
                max_block_header_size,
                key_deposit,
                pool_deposit,
                max_epoch,
                n_opt,
                pool_pledge_influence,
                expansion_rate,
                treasury_growth_rate,
                d,
                extra_entropy,
                protocol_version,
                min_pool_cost,
                ada_per_utxo_byte,
                cost_models,
                execution_costs,
                max_tx_ex_units,
                max_block_ex_units,
                max_value_size,
                collateral_percentage,
                max_collateral_inputs,
                pool_voting_thresholds,
                drep_voting_thresholds,
                min_committee_size,
                committee_term_limit,
                governance_action_validity_period,
                governance_action_deposit,
                drep_deposit,
                drep_inactivity_period,
                ref_script_coins_per_byte,
            })
        })()
        .map_err(|e| e.annotate("ProtocolParamUpdate"))
    }
}
