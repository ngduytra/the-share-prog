use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::schema::request::*;
use crate::utils::*;
use crate::Plan;

#[event]
pub struct CreateRequestEvent {
    pub withdrawer: Pubkey,
    pub plan: Pubkey,
    pub amount: u64,
    pub reason: String,
    pub time: i64,
}

#[derive(Accounts)]
pub struct CreateRequest<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,
    #[account(init, payer = withdrawer, space = Request::LEN)]
    pub request: Account<'info, Request>,
    #[account(mut)]
    pub plan: Account<'info, Plan>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn exec(ctx: Context<CreateRequest>, amount: u64, reason: &String) -> Result<()> {
    let plan = &mut ctx.accounts.plan;
    let request = &mut ctx.accounts.request;

    if amount <= 0 || amount > plan.fund {
        return err!(ErrorCode::InvalidAmount);
    }
    if !plan
        .withdrawer_list
        .contains(&ctx.accounts.withdrawer.key())
    {
        return err!(ErrorCode::InvalidAuthorization);
    }

    let time = current_timestamp().ok_or(ErrorCode::Overflow)?;

    // Create plan data
    request.plan = ctx.accounts.plan.key();
    request.withdrawer = ctx.accounts.withdrawer.key();
    request.amount = amount;
    request.state = RequestState::Initialized;
    request.reason = reason.to_string();
    request.time = time;

    emit!(CreateRequestEvent {
        withdrawer: ctx.accounts.withdrawer.key(),
        plan: ctx.accounts.plan.key(),
        amount,
        reason: reason.to_string(),
        time,
    });

    Ok(())
}
