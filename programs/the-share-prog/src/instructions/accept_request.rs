use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::errors::ErrorCode;
use crate::schema::request::*;
use crate::utils::*;
use crate::Plan;

#[event]
pub struct AcceptRequestEvent {
    pub withdrawer: Pubkey,
    pub amount: u64,
    pub reason: String,
    time: i64,
}

#[derive(Accounts)]
pub struct AcceptRequest<'info> {
    #[account(mut)]
    pub planer: Signer<'info>,
    #[account(mut)]
    pub request: Account<'info, Request>,
    #[account(mut)]
    pub ata_planer: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub ata_requester: Account<'info, token::TokenAccount>,
    #[account(mut, has_one=planer@ErrorCode::InvalidAuthorization)]
    pub plan: Account<'info, Plan>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
}

pub fn exec(ctx: Context<AcceptRequest>) -> Result<()> {
    let plan = &mut ctx.accounts.plan;
    let request = &mut ctx.accounts.request;

    if request.amount > plan.fund {
        return err!(ErrorCode::InvalidAmount);
    }

    if request.state != RequestState::Initialized {
        return err!(ErrorCode::InvalidAuthorization);
    }

    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.ata_planer.to_account_info(),
            to: ctx.accounts.ata_requester.to_account_info(),
            authority: ctx.accounts.planer.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, request.amount)?;

    let remaining_fund = plan
        .accept_request(request.amount)
        .ok_or(ErrorCode::Overflow)?;

    request.state = RequestState::Confirmed;
    plan.fund = remaining_fund;

    let time = current_timestamp().ok_or(ErrorCode::Overflow)?;

    emit!(AcceptRequestEvent {
        withdrawer: request.withdrawer,
        amount: request.amount,
        reason: request.reason.to_string(),
        time,
    });

    Ok(())
}
