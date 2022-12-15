use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::schema::request::*;
use crate::utils::*;
use crate::Plan;

#[event]
pub struct CancelRequestEvent {
    pub withdrawer: Pubkey,
    pub plan: Pubkey,
    pub amount: u64,
    pub reason: String,
    pub time: i64,
}

#[derive(Accounts)]
pub struct CancelRequest<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,
    #[account(mut, has_one = withdrawer)]
    pub request: Account<'info, Request>,
    #[account(mut)]
    pub plan: Account<'info, Plan>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn exec(ctx: Context<CancelRequest>) -> Result<()> {
    let plan = &mut ctx.accounts.plan;
    let request = &mut ctx.accounts.request;

    if !plan
        .withdrawer_list
        .contains(&ctx.accounts.withdrawer.key())
        || request.state != RequestState::Initialized
    {
        return err!(ErrorCode::InvalidAuthorization);
    }

    let time = current_timestamp().ok_or(ErrorCode::Overflow)?;

    request.state = RequestState::Canceled;

    emit!(CancelRequestEvent {
        withdrawer: ctx.accounts.withdrawer.key(),
        plan: ctx.accounts.plan.key(),
        amount: request.amount,
        reason: request.reason.to_string(),
        time,
    });

    Ok(())
}
