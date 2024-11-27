use crate::*;
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub(crate) enum TxBodyNames {
    Inputs = 0,
    Outputs = 1,
    Fee = 2,
    Ttl = 3,
    Certs = 4,
    Withdrawals = 5,
    Update = 6,
    AuxiliaryDataHash = 7,
    ValidityStartInterval = 8,
    Mint = 9,
    ScriptDataHash = 11,
    Collateral = 13,
    RequiredSigners = 14,
    NetworkId = 15,
    CollateralReturn = 16,
    TotalCollateral = 17,
    ReferenceInputs = 18,
}

impl TxBodyNames {
    pub fn to_u64(&self) -> Option<u64> {
        Some(self.clone() as u64)
    }

    pub fn from_u64(value: u64) -> Option<Self> {
        match value {
            0 => Some(TxBodyNames::Inputs),
            1 => Some(TxBodyNames::Outputs),
            2 => Some(TxBodyNames::Fee),
            3 => Some(TxBodyNames::Ttl),
            4 => Some(TxBodyNames::Certs),
            5 => Some(TxBodyNames::Withdrawals),
            6 => Some(TxBodyNames::Update),
            7 => Some(TxBodyNames::AuxiliaryDataHash),
            8 => Some(TxBodyNames::ValidityStartInterval),
            9 => Some(TxBodyNames::Mint),
            11 => Some(TxBodyNames::ScriptDataHash),
            13 => Some(TxBodyNames::Collateral),
            14 => Some(TxBodyNames::RequiredSigners),
            15 => Some(TxBodyNames::NetworkId),
            16 => Some(TxBodyNames::CollateralReturn),
            17 => Some(TxBodyNames::TotalCollateral),
            18 => Some(TxBodyNames::ReferenceInputs),
            _ => None,
        }
    }
}
