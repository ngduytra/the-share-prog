use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;

pub mod schema;
pub use schema::*;

pub mod errors;
pub use errors::*;

pub mod utils;
pub use utils::*;

pub mod constants;

declare_id!("CGvPBsRDLFxd4YaKcgfJFpWMHo5vc8DGdpJNfH9b9y87");

#[program]
pub mod the_share_prog {
    use super::*;

    pub fn create_plan(
        ctx: Context<CreatePlan>,
        fund: u64,
        plan_name: String,
        withdrawer_list: Vec<Pubkey>,
    ) -> Result<()> {
        create_plan::exec(ctx, fund, &plan_name, withdrawer_list)
    }

    pub fn change_plan_configs(
        ctx: Context<ChangePlanConfigs>,
        fund: u64,
        plan_name: String,
        withdrawer_list: Vec<Pubkey>,
    ) -> Result<()> {
        change_plan_configs::exec(ctx, fund, &plan_name, withdrawer_list)
    }

    pub fn create_request(ctx: Context<CreateRequest>, amount: u64, reason: String) -> Result<()> {
        create_request::exec(ctx, amount, &reason)
    }

    pub fn change_request(ctx: Context<ChangeRequest>, amount: u64, reason: String) -> Result<()> {
        change_request::exec(ctx, amount, &reason)
    }

    pub fn accept_request(ctx: Context<AcceptRequest>) -> Result<()> {
        accept_request::exec(ctx)
    }

    pub fn reject_request(ctx: Context<RejectRequest>) -> Result<()> {
        reject_request::exec(ctx)
    }

    pub fn cancel_request(ctx: Context<CancelRequest>) -> Result<()> {
        cancel_request::exec(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
