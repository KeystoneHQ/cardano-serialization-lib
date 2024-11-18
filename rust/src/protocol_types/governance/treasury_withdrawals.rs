use crate::*;
use alloc::collections::BTreeMap;
use serialization::is_break_tag;

#[derive(
    Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, serde::Serialize, serde::Deserialize,
)]
#[wasm_bindgen]
pub struct TreasuryWithdrawals(pub(crate) BTreeMap<RewardAddress, Coin>);

to_from_json!(TreasuryWithdrawals);

#[wasm_bindgen]
impl TreasuryWithdrawals {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn get(&self, key: &RewardAddress) -> Option<Coin> {
        self.0.get(key).cloned()
    }

    pub fn insert(&mut self, key: &RewardAddress, value: &Coin) {
        self.0.insert(key.clone(), value.clone());
    }

    pub fn keys(&self) -> RewardAddresses {
        RewardAddresses(self.0.keys().cloned().collect())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Serialize for TreasuryWithdrawals {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(self.0.len() as u64))?;
        for (key, value) in &self.0 {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for TreasuryWithdrawals {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut table = BTreeMap::new();
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            while match len {
                cbor_event::Len::Len(n) => table.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "TreasuryWithdrawals")? {
                    break;
                }
                let key = RewardAddress::deserialize(raw)?;
                let value = Coin::deserialize(raw)?;
                if table.insert(key.clone(), value).is_some() {
                    return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                        "some complicated/unsupported type",
                    )))
                    .into());
                }
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("TreasuryWithdrawals"))?;
        Ok(Self(table))
    }
}
