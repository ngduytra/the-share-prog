use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::schema::request::*;
use crate::Plan;

#[event]
pub struct RejectRequestEvent {
    pub withdrawer: Pubkey,
    pub plan: Pubkey,
    pub amount: u64,
    pub reason: String,
}

#[derive(Accounts)]
pub struct RejectRequest<'info> {
    #[account(mut)]
    pub planer: Signer<'info>,
    #[account(mut, has_one=planer)]
    pub plan: Account<'info, Plan>,
    #[account(mut)]
    pub request: Account<'info, Request>,

    pub system_program: Program<'info, System>,
}

pub fn exec(ctx: Context<RejectRequest>) -> Result<()> {
    let request = &mut ctx.accounts.request;

    if request.state != RequestState::Initialized {
        return err!(ErrorCode::InvalidAuthorization);
    }

    request.state = RequestState::Rejected;

    emit!(RejectRequestEvent {
        withdrawer: request.withdrawer.key(),
        plan: ctx.accounts.plan.key(),
        amount: request.amount,
        reason: request.reason.to_string(),
    });

    Ok(())
}
