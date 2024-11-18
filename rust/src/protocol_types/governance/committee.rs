use crate::*;
use alloc::collections::BTreeMap;
use serialization::{check_len, is_break_tag};

#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, serde::Serialize, serde::Deserialize,
)]
struct CommitteeMember {
    stake_credential: StakeCredential,
    term_limit: Epoch,
}

#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, serde::Serialize, serde::Deserialize,
)]
struct CommitteeJsonStruct {
    members: Vec<CommitteeMember>,
    quorum_threshold: UnitInterval,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[wasm_bindgen]
pub struct Committee {
    pub(crate) members: BTreeMap<StakeCredential, Epoch>,
    pub(crate) quorum_threshold: UnitInterval,
}

impl_to_from!(Committee);

#[wasm_bindgen]
impl Committee {
    pub fn new(quorum_threshold: &UnitInterval) -> Self {
        Self {
            members: BTreeMap::new(),
            quorum_threshold: quorum_threshold.clone(),
        }
    }

    pub fn members_keys(&self) -> StakeCredentials {
        StakeCredentials::from_iter(self.members.keys().cloned())
    }

    pub fn quorum_threshold(&self) -> UnitInterval {
        self.quorum_threshold.clone()
    }

    pub fn add_member(&mut self, committee_cold_credential: &StakeCredential, epoch: Epoch) {
        self.members
            .insert(committee_cold_credential.clone(), epoch);
    }

    pub fn get_member_epoch(&self, committee_cold_credential: &StakeCredential) -> Option<Epoch> {
        self.members.get(committee_cold_credential).cloned()
    }
}

impl serde::ser::Serialize for Committee {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let committee = CommitteeJsonStruct {
            members: self
                .members
                .iter()
                .map(|(k, v)| CommitteeMember {
                    stake_credential: k.clone(),
                    term_limit: v.clone(),
                })
                .collect(),
            quorum_threshold: self.quorum_threshold.clone(),
        };

        committee.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for Committee {
    fn deserialize<D>(deserializer: D) -> Result<Committee, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let committee_json: CommitteeJsonStruct =
            serde::de::Deserialize::deserialize(deserializer)?;
        let mut committee = Committee::new(&committee_json.quorum_threshold);
        let mut members = BTreeMap::new();
        for member in committee_json.members {
            members.insert(member.stake_credential, member.term_limit);
        }
        committee.members = members;
        Ok(committee)
    }
}

impl Serialize for Committee {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.serialize_as_embedded_group(serializer)?;
        Ok(serializer)
    }
}

impl SerializeEmbeddedGroup for Committee {
    fn serialize_as_embedded_group<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.members.len() as u64))?;
        for (key, value) in &self.members {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        self.quorum_threshold.serialize(serializer)?;
        Ok(serializer)
    }
}

impl_deserialize_for_wrapped_tuple!(Committee);

impl DeserializeEmbeddedGroup for Committee {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        check_len(len, 2, "(members, quorum_threshold)")?;

        let mut table = BTreeMap::new();
        let map_len = raw.map()?;
        while match map_len {
            cbor_event::Len::Len(n) => table.len() < n as usize,
            cbor_event::Len::Indefinite => true,
        } {
            if is_break_tag(raw, "Committee")? {
                break;
            }
            let key = StakeCredential::deserialize(raw)?;
            let value = Epoch::deserialize(raw)?;
            if table.insert(key.clone(), value).is_some() {
                return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                    "some complicated/unsupported type",
                )))
                .into());
            }
        }
        let quorum_threshold = UnitInterval::deserialize(raw)?;

        Ok(Committee {
            quorum_threshold,
            members: table,
        })
    }
}
