#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub(crate) enum CertificateIndexNames {
    StakeRegistrationLegacy = 0,
    StakeDeregistrationLegacy = 1,
    StakeDelegation = 2,
    PoolRegistration = 3,
    PoolRetirement = 4,
    GenesisKeyDelegation = 5,
    MoveInstantaneousRewardsCert = 6,
    StakeRegistrationConway = 7,
    StakeDeregistrationConway = 8,
    VoteDelegation = 9,
    StakeAndVoteDelegation = 10,
    StakeRegistrationAndDelegation = 11,
    VoteRegistrationAndDelegation = 12,
    StakeVoteRegistrationAndDelegation = 13,
    CommitteeHotAuth = 14,
    CommitteeColdResign = 15,
    DrepRegistration = 16,
    DrepDeregistration = 17,
    DrepUpdate = 18,
}

impl CertificateIndexNames {
    pub fn from_u64(value: u64) -> Option<Self> {
        match value {
            0 => Some(CertificateIndexNames::StakeRegistrationLegacy),
            1 => Some(CertificateIndexNames::StakeDeregistrationLegacy),
            2 => Some(CertificateIndexNames::StakeDelegation),
            3 => Some(CertificateIndexNames::PoolRegistration),
            4 => Some(CertificateIndexNames::PoolRetirement),
            5 => Some(CertificateIndexNames::GenesisKeyDelegation),
            6 => Some(CertificateIndexNames::MoveInstantaneousRewardsCert),
            7 => Some(CertificateIndexNames::StakeRegistrationConway),
            8 => Some(CertificateIndexNames::StakeDeregistrationConway),
            9 => Some(CertificateIndexNames::VoteDelegation),
            10 => Some(CertificateIndexNames::StakeAndVoteDelegation),
            11 => Some(CertificateIndexNames::StakeRegistrationAndDelegation),
            12 => Some(CertificateIndexNames::VoteRegistrationAndDelegation),
            13 => Some(CertificateIndexNames::StakeVoteRegistrationAndDelegation),
            14 => Some(CertificateIndexNames::CommitteeHotAuth),
            15 => Some(CertificateIndexNames::CommitteeColdResign),
            16 => Some(CertificateIndexNames::DrepRegistration),
            17 => Some(CertificateIndexNames::DrepDeregistration),
            18 => Some(CertificateIndexNames::DrepUpdate),
            _ => None,
        }
    }
}
