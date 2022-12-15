use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::errors::ErrorCode;
use crate::schema::plan::*;

#[event]
pub struct ChangePlanConfigsEvent {
    pub planer: Pubkey,
    pub plan_name: String,
    pub fund: u64,
    pub token: Pubkey,
}

#[derive(Accounts)]
pub struct ChangePlanConfigs<'info> {
    #[account(mut)]
    pub planer: Signer<'info>,
    #[account(mut, has_one=planer@ErrorCode::InvalidAuthorization)]
    pub plan: Account<'info, Plan>,
    #[account(mut)]
    pub ata_planer: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub treasury: Box<Account<'info, token::TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
}

pub fn exec(
    ctx: Context<ChangePlanConfigs>,
    fund: u64,
    plan_name: &String,
    withdrawer_list: Vec<Pubkey>,
) -> Result<()> {
    let plan = &mut ctx.accounts.plan;
    if fund <= 0 {
        return err!(ErrorCode::InvalidAmount);
    }

    let approve_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Approve {
            to: ctx.accounts.ata_planer.to_account_info(),
            delegate: ctx.accounts.treasury.to_account_info(),
            authority: ctx.accounts.planer.to_account_info(),
        },
    );
    token::approve(approve_ctx, fund)?;

    // Create pool data
    plan.plan_name = plan_name.to_string();
    plan.withdrawer_list = withdrawer_list;
    plan.fund = fund;

    emit!(ChangePlanConfigsEvent {
        planer: ctx.accounts.planer.key(),
        plan_name: plan_name.to_string(),
        fund,
        token: plan.token,
    });

    Ok(())
}
