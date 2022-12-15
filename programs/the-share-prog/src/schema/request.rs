use crate::constants::*;
use anchor_lang::prelude::*;

///
/// Request State
///
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum RequestState {
    Initialized,
    Confirmed,
    Rejected,
    Canceled,
}

#[account]
pub struct Request {
    pub withdrawer: Pubkey,
    pub plan: Pubkey,
    pub amount: u64,
    pub state: RequestState,
    pub reason: String,
    pub time: i64,
}

impl Request {
    pub const LEN: usize =
        DISCRIMINATOR_SIZE + 2 * PUBKEY_SIZE + I64_SIZE + U64_SIZE + 4 + STRING_SIZE + U8_SIZE;
}
