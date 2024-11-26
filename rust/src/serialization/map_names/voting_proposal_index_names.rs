#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub(crate) enum VotingProposalIndexNames {
    ParameterChangeAction = 0,
    HardForkInitiationAction = 1,
    TreasuryWithdrawalsAction = 2,
    NoConfidenceAction = 3,
    UpdateCommitteeAction = 4,
    NewConstitutionAction = 5,
    InfoAction = 6,
}

impl VotingProposalIndexNames {
    pub fn to_u64(&self) -> Option<u64> {
        Some(self.clone() as u64)
    }

    pub fn from_u64(value: u64) -> Option<Self> {
        match value {
            0 => Some(VotingProposalIndexNames::ParameterChangeAction),
            1 => Some(VotingProposalIndexNames::HardForkInitiationAction),
            2 => Some(VotingProposalIndexNames::TreasuryWithdrawalsAction),
            3 => Some(VotingProposalIndexNames::NoConfidenceAction),
            4 => Some(VotingProposalIndexNames::UpdateCommitteeAction),
            5 => Some(VotingProposalIndexNames::NewConstitutionAction),
            6 => Some(VotingProposalIndexNames::InfoAction),
            _ => None,
        }
    }
}
