use casper_contract::contract_api::runtime::revert;

use crate::error::NotarizerError;

#[derive(Clone, Copy)]
pub enum AssociateType {
    NotMember = 0,
    Partner = 1,
    Judge = 2,
    Lawyer = 3,
    Clerk = 4,
}

impl From<AssociateType> for u32 {
    fn from(fin: AssociateType) -> u32 {
        fin as u32
    }
}

impl From<u32> for AssociateType {
    fn from(u: u32) -> AssociateType {
        match u {
            0 => AssociateType::NotMember,
            1 => AssociateType::Partner,
            2 => AssociateType::Judge,
            3 => AssociateType::Lawyer,
            4 => AssociateType::Clerk,
            _ => revert(NotarizerError::AssociateTypeNonExistant),
        }
    }
}
