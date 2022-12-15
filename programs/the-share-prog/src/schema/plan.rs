use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct Plan {
    pub planer: Pubkey,
    pub plan_name: String,
    pub withdrawer_list: Vec<Pubkey>,
    pub fund: u64,
    pub token: Pubkey,
}

impl Plan {
    pub const LEN: usize = DISCRIMINATOR_SIZE
        + PUBKEY_SIZE
        + 4
        + STRING_SIZE
        + 4
        + 10 * PUBKEY_SIZE
        + U64_SIZE
        + PUBKEY_SIZE;

    pub fn accept_request(&self, amount: u64) -> Option<u64> {
        let a = self.fund.checked_sub(amount)?;
        Some(a)
    }
}
