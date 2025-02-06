use anchor_lang::prelude::*;
mod instructions;
use instructions::initialize::*;
use instructions::add::*;
mod state;
mod constant;
mod error;
declare_id!("8F4EfF73Yq2guFhw69v4i57CVCP9kawuWaGWkLGyMfNa");

#[program]
pub mod todolist {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, max_length: u8) -> Result<()> {
        handle_initialize(ctx, max_length)
    }

    pub fn add(ctx: Context<Add>, content: String) -> Result<()> {
        handle_add(ctx, content)
    }
}

 
