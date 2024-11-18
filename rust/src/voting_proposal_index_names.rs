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

    pub fn to_u64(&self) -> Option<u64> {
        match self {
            VotingProposalIndexNames::ParameterChangeAction => Some(0),
            VotingProposalIndexNames::HardForkInitiationAction => Some(1),
            VotingProposalIndexNames::TreasuryWithdrawalsAction => Some(2),
            VotingProposalIndexNames::NoConfidenceAction => Some(3),
            VotingProposalIndexNames::UpdateCommitteeAction => Some(4),
            VotingProposalIndexNames::NewConstitutionAction => Some(5),
            VotingProposalIndexNames::InfoAction => Some(6),
        }
    }
}
