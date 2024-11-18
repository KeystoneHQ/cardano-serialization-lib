use crate::*;

#[derive(
    Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub(crate) enum VoterEnum {
    ConstitutionalCommitteeHotKey(StakeCredential),
    DRep(StakeCredential),
    StakingPool(Ed25519KeyHash),
}

#[wasm_bindgen]
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum VoterKind {
    ConstitutionalCommitteeHotKeyHash,
    ConstitutionalCommitteeHotScriptHash,
    DRepKeyHash,
    DRepScriptHash,
    StakingPoolKeyHash,
}

#[derive(
    Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[wasm_bindgen]
pub struct Voter(pub(crate) VoterEnum);

impl_to_from!(Voter);

#[wasm_bindgen]
impl Voter {
    pub fn new_constitutional_committee_hot_key(cred: &StakeCredential) -> Self {
        Self(VoterEnum::ConstitutionalCommitteeHotKey(cred.clone()))
    }

    pub fn new_drep(cred: &StakeCredential) -> Self {
        Self(VoterEnum::DRep(cred.clone()))
    }

    pub fn new_staking_pool(key_hash: &Ed25519KeyHash) -> Self {
        Self(VoterEnum::StakingPool(key_hash.clone()))
    }

    pub fn kind(&self) -> VoterKind {
        match &self.0 {
            VoterEnum::ConstitutionalCommitteeHotKey(cred) => match cred.kind() {
                StakeCredKind::Key => VoterKind::ConstitutionalCommitteeHotKeyHash,
                StakeCredKind::Script => VoterKind::ConstitutionalCommitteeHotScriptHash,
            },
            VoterEnum::DRep(cred) => match cred.kind() {
                StakeCredKind::Key => VoterKind::DRepKeyHash,
                StakeCredKind::Script => VoterKind::DRepScriptHash,
            },
            VoterEnum::StakingPool(_) => VoterKind::StakingPoolKeyHash,
        }
    }

    pub fn to_constitutional_committee_hot_cred(&self) -> Option<StakeCredential> {
        match &self.0 {
            VoterEnum::ConstitutionalCommitteeHotKey(cred) => Some(cred.clone()),
            _ => None,
        }
    }

    pub fn to_drep_cred(&self) -> Option<StakeCredential> {
        match &self.0 {
            VoterEnum::DRep(cred) => Some(cred.clone()),
            _ => None,
        }
    }

    pub fn to_staking_pool_key_hash(&self) -> Option<Ed25519KeyHash> {
        match &self.0 {
            VoterEnum::StakingPool(key_hash) => Some(key_hash.clone()),
            _ => None,
        }
    }

    pub fn has_script_credentials(&self) -> bool {
        match &self.0 {
            VoterEnum::ConstitutionalCommitteeHotKey(cred) => cred.has_script_hash(),
            VoterEnum::DRep(cred) => cred.has_script_hash(),
            VoterEnum::StakingPool(_) => false,
        }
    }

    pub fn to_key_hash(&self) -> Option<Ed25519KeyHash> {
        match &self.0 {
            VoterEnum::ConstitutionalCommitteeHotKey(cred) => cred.to_keyhash(),
            VoterEnum::DRep(cred) => cred.to_keyhash(),
            VoterEnum::StakingPool(key_hash) => Some(key_hash.clone()),
        }
    }
}

impl cbor_event::se::Serialize for Voter {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for Voter {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let voter_enum = VoterEnum::deserialize(raw)?;
            Ok(Self(voter_enum))
        })()
        .map_err(|e| e.annotate("Voter"))
    }
}

impl cbor_event::se::Serialize for VoterEnum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        match &self {
            VoterEnum::ConstitutionalCommitteeHotKey(cred) => match &cred.0 {
                StakeCredType::Key(key_hash) => {
                    serializer.write_unsigned_integer(0u64)?;
                    key_hash.serialize(serializer)?;
                }
                StakeCredType::Script(script_hash) => {
                    serializer.write_unsigned_integer(1u64)?;
                    script_hash.serialize(serializer)?;
                }
            },
            VoterEnum::DRep(cred) => match &cred.0 {
                StakeCredType::Key(key_hash) => {
                    serializer.write_unsigned_integer(2u64)?;
                    key_hash.serialize(serializer)?;
                }
                StakeCredType::Script(script_hash) => {
                    serializer.write_unsigned_integer(3u64)?;
                    script_hash.serialize(serializer)?;
                }
            },
            VoterEnum::StakingPool(scripthash) => {
                serializer.write_unsigned_integer(4u64)?;
                scripthash.serialize(serializer)?;
            }
        };
        Ok(serializer)
    }
}

impl Deserialize for VoterEnum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            if let cbor_event::Len::Len(n) = len {
                if n != 2 {
                    return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                        2,
                        len,
                        "[id, hash]",
                    ))
                    .into());
                }
            }
            let voter = match raw.unsigned_integer()? {
                0 => VoterEnum::ConstitutionalCommitteeHotKey(StakeCredential(StakeCredType::Key(
                    Ed25519KeyHash::deserialize(raw)?,
                ))),
                1 => VoterEnum::ConstitutionalCommitteeHotKey(StakeCredential(
                    StakeCredType::Script(ScriptHash::deserialize(raw)?),
                )),
                2 => VoterEnum::DRep(StakeCredential(StakeCredType::Key(
                    Ed25519KeyHash::deserialize(raw)?,
                ))),
                3 => VoterEnum::DRep(StakeCredential(StakeCredType::Script(
                    ScriptHash::deserialize(raw)?,
                ))),
                4 => VoterEnum::StakingPool(Ed25519KeyHash::deserialize(raw)?),
                n => {
                    return Err(DeserializeFailure::FixedValuesMismatch {
                        found: Key::Uint(n),
                        expected: vec![
                            Key::Uint(0),
                            Key::Uint(1),
                            Key::Uint(2),
                            Key::Uint(3),
                            Key::Uint(4),
                        ],
                    }
                    .into())
                }
            };
            if let cbor_event::Len::Indefinite = len {
                if raw.special()? != CBORSpecial::Break {
                    return Err(DeserializeFailure::EndingBreakMissing.into());
                }
            }
            Ok(voter)
        })()
        .map_err(|e| e.annotate("VoterEnum"))
    }
}
