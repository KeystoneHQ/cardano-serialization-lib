use crate::*;
use serialization::{is_break_tag, skip_set_tag};

#[derive(
    Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
#[wasm_bindgen]
pub struct VotingProposals(pub(crate) Vec<VotingProposal>);

impl_to_from!(VotingProposals);

impl NoneOrEmpty for VotingProposals {
    fn is_none_or_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[wasm_bindgen]
impl VotingProposals {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> VotingProposal {
        self.0[index].clone()
    }

    pub fn add(&mut self, proposal: &VotingProposal) {
        self.0.push(proposal.clone());
    }
}

impl cbor_event::se::Serialize for VotingProposals {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        //TODO: uncomment this line when we conway ero will come
        //serializer.write_tag(258)?;
        serializer.write_array(cbor_event::Len::Len(self.0.len() as u64))?;
        for element in &self.0 {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for VotingProposals {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut arr = Vec::new();
        (|| -> Result<_, DeserializeError> {
            skip_set_tag(raw)?;
            let len = raw.array()?;
            while match len {
                cbor_event::Len::Len(n) => arr.len() < n as usize,
                cbor_event::Len::Indefinite => true,
            } {
                if is_break_tag(raw, "VotingProposals")? {
                    break;
                }
                arr.push(VotingProposal::deserialize(raw)?);
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("VotingProposals"))?;
        Ok(Self(arr))
    }
}
