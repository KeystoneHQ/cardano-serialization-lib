#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub(crate) enum WitnessSetNames {
    Vkeys = 0,
    NativeScripts = 1,
    Bootstraps = 2,
    PlutusScriptsV1 = 3,
    PlutusData = 4,
    Redeemers = 5,
    PlutusScriptsV2 = 6,
    PlutusScriptsV3 = 7,
}

impl WitnessSetNames {
    pub fn to_u64(&self) -> Option<u64> {
        Some(self.clone() as u64)
    }

    pub fn from_u64(value: u64) -> Option<Self> {
        match value {
            0 => Some(WitnessSetNames::Vkeys),
            1 => Some(WitnessSetNames::NativeScripts),
            2 => Some(WitnessSetNames::Bootstraps),
            3 => Some(WitnessSetNames::PlutusScriptsV1),
            4 => Some(WitnessSetNames::PlutusData),
            5 => Some(WitnessSetNames::Redeemers),
            6 => Some(WitnessSetNames::PlutusScriptsV2),
            7 => Some(WitnessSetNames::PlutusScriptsV3),
            _ => None,
        }
    }
}
