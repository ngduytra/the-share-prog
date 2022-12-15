use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};

use crate::errors::ErrorCode;
use crate::schema::plan::*;

#[event]
pub struct CreatePlanEvent {
    pub planer: Pubkey,
    pub plan_name: String,
    pub fund: u64,
    pub token: Pubkey,
}

#[derive(Accounts)]
pub struct CreatePlan<'info> {
    #[account(mut)]
    pub planer: Signer<'info>,
    #[account(init, payer = planer, space = Plan::LEN)]
    pub plan: Account<'info, Plan>,
    pub token: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub ata_planer: Account<'info, token::TokenAccount>,
    #[account(seeds = [b"treasurer", &plan.key().to_bytes()], bump)]
    /// CHECK: Just a pure account
    pub treasurer: AccountInfo<'info>,
    #[account(
    init,
    payer = planer,
    associated_token::mint = token,
    associated_token::authority = treasurer
  )]
    pub treasury: Box<Account<'info, token::TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn exec(
    ctx: Context<CreatePlan>,
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
    plan.planer = ctx.accounts.planer.key();
    plan.withdrawer_list = withdrawer_list;
    plan.token = ctx.accounts.token.key();
    plan.fund = fund;

    emit!(CreatePlanEvent {
        planer: ctx.accounts.planer.key(),
        plan_name: plan_name.to_string(),
        fund,
        token: ctx.accounts.token.key(),
    });

    Ok(())
}
