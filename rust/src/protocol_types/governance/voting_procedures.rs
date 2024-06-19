use crate::*;
use serde::ser::SerializeSeq;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use serialization::is_break_tag;

#[derive(
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
struct VoterVotes {
    voter: Voter,
    votes: BTreeSet<Vote>,
}

#[derive(
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
struct Vote {
    action_id: GovernanceActionId,
    voting_procedure: VotingProcedure,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[wasm_bindgen]
pub struct VotingProcedures(
    pub(crate) BTreeMap<Voter, BTreeMap<GovernanceActionId, VotingProcedure>>,
);

impl_to_from!(VotingProcedures);

impl NoneOrEmpty for VotingProcedures {
    fn is_none_or_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[wasm_bindgen]
impl VotingProcedures {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(
        &mut self,
        voter: &Voter,
        governance_action_id: &GovernanceActionId,
        voting_procedure: &VotingProcedure,
    ) {
        self.0
            .entry(voter.clone())
            .or_insert_with(BTreeMap::new)
            .insert(governance_action_id.clone(), voting_procedure.clone());
    }

    pub fn get(
        &self,
        voter: &Voter,
        governance_action_id: &GovernanceActionId,
    ) -> Option<VotingProcedure> {
        self.0
            .get(voter)
            .and_then(|v| v.get(governance_action_id))
            .cloned()
    }

    pub fn get_voters(&self) -> Voters {
        Voters(self.0.keys().cloned().collect())
    }

    pub fn get_governance_action_ids_by_voter(&self, voter: &Voter) -> GovernanceActionIds {
        GovernanceActionIds(
            self.0
                .get(voter)
                .map(|v| v.keys().cloned().collect())
                .unwrap_or_default(),
        )
    }
}

impl serde::ser::Serialize for VotingProcedures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for (voter, votes) in &self.0 {
            let voter_votes = VoterVotes {
                voter: voter.clone(),
                votes: votes
                    .iter()
                    .map(|(action_id, voting_procedure)| Vote {
                        action_id: action_id.clone(),
                        voting_procedure: voting_procedure.clone(),
                    })
                    .collect(),
            };
            seq.serialize_element(&voter_votes)?;
        }
        seq.end()
    }
}

impl<'de> serde::de::Deserialize<'de> for VotingProcedures {
    fn deserialize<D>(deserializer: D) -> Result<VotingProcedures, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let all_votes: Vec<VoterVotes> = serde::de::Deserialize::deserialize(deserializer)?;
        let mut voting_procedures = VotingProcedures::new();
        for votes in all_votes {
            let mut voter_votes = BTreeMap::new();
            for vote in votes.votes {
                voter_votes.insert(vote.action_id, vote.voting_procedure);
            }
            voting_procedures.0.insert(votes.voter, voter_votes);
        }
        Ok(voting_procedures)
    }
}


impl cbor_event::se::Serialize for VotingProcedures {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64))?;

        for (voter, votes) in &self.0 {
            voter.serialize(serializer)?;
            serializer.write_map(cbor_event::Len::Len(votes.len() as u64))?;
            for (governance_action_id, voting_procedure) in votes {
                governance_action_id.serialize(serializer)?;
                voting_procedure.serialize(serializer)?;
            }
        }
        Ok(serializer)
    }
}

impl Deserialize for VotingProcedures {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut voter_to_vote = BTreeMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len {
                cbor_event::Len::Len(n) => voter_to_vote.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "voting_procedure map")? {
                    break;
                }

                let key = Voter::deserialize(raw).map_err(|e| e.annotate("voter"))?;

                let value = deserialize_internal_map(raw)
                    .map_err(|e| e.annotate("voting_procedure map"))?;

                if voter_to_vote.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                        "some complicated/unsupported type",
                    )))
                    .into());
                }
            }
            Ok(Self(voter_to_vote))
        })()
        .map_err(|e| e.annotate("VotingProcedures"))
    }
}

fn deserialize_internal_map<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
) -> Result<BTreeMap<GovernanceActionId, VotingProcedure>, DeserializeError> {
    let mut gov_act_id_to_vote = BTreeMap::new();
    (|| -> Result<_, DeserializeError> {
        let len = raw.map()?;
        while match len {
            cbor_event::Len::Len(n) => gov_act_id_to_vote.len() < n as usize,
            cbor_event::Len::Indefinite => true,
        } {
            if is_break_tag(raw, "gov_act_id_to_vote map")? {
                break;
            }

            let key = GovernanceActionId::deserialize(raw).map_err(|e| e.annotate("gov_act_id"))?;

            let value =
                VotingProcedure::deserialize(raw).map_err(|e| e.annotate("voting_procedure"))?;

            if gov_act_id_to_vote.insert(key.clone(), value).is_some() {
                return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                    "some complicated/unsupported type",
                )))
                .into());
            }
        }
        Ok(gov_act_id_to_vote)
    })()
    .map_err(|e| e.annotate("VotingProcedures (gov_act_id to vote_procedure map)"))
}
