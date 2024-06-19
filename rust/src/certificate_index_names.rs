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

    pub fn to_u64(&self) -> Option<u64> {
        match self {
            CertificateIndexNames::StakeRegistrationLegacy => Some(0),
            CertificateIndexNames::StakeDeregistrationLegacy => Some(1),
            CertificateIndexNames::StakeDelegation => Some(2),
            CertificateIndexNames::PoolRegistration => Some(3),
            CertificateIndexNames::PoolRetirement => Some(4),
            CertificateIndexNames::GenesisKeyDelegation => Some(5),
            CertificateIndexNames::MoveInstantaneousRewardsCert => Some(6),
            CertificateIndexNames::StakeRegistrationConway => Some(7),
            CertificateIndexNames::StakeDeregistrationConway => Some(8),
            CertificateIndexNames::VoteDelegation => Some(9),
            CertificateIndexNames::StakeAndVoteDelegation => Some(10),
            CertificateIndexNames::StakeRegistrationAndDelegation => Some(11),
            CertificateIndexNames::VoteRegistrationAndDelegation => Some(12),
            CertificateIndexNames::StakeVoteRegistrationAndDelegation => Some(13),
            CertificateIndexNames::CommitteeHotAuth => Some(14),
            CertificateIndexNames::CommitteeColdResign => Some(15),
            CertificateIndexNames::DrepRegistration => Some(16),
            CertificateIndexNames::DrepDeregistration => Some(17),
            CertificateIndexNames::DrepUpdate => Some(18),
        }
    }
}
