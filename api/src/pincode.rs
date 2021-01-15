use crate::account::db::AccountId;

#[repr(u16)]
#[derive(Copy, Clone, Debug, int_enum::IntEnum)]
pub enum PincodeStatus {
    Correct = 0,
    AskForPin = 1,
    PinMustBeChanged = 2,
    NeedNewPin = 3,
    CreateNewPin = 4,
    // TODO: update my description
    ClientWarning = 5,
    UnableToUseKSSNNumber = 6,
    ShowButton = 7,
    Incorrect = 8,
}

#[derive(Clone, Copy, Debug)]
pub struct PincodeInfo {
    pub status: PincodeStatus,
    pub account_id: AccountId,
}
