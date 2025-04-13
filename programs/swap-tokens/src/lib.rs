pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("VPq5xwCQ5v1p2ZD5rca3H6oPRgjk3o4AjmEEPECQnpC");

#[program]
pub mod swap_tokens {
    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        id: u64,
        token_a_amount: u64, // offered token
        token_b_amount: u64, // wanted token
    ) -> Result<()> {
        make_offer::send_tokens_to_vault(&ctx, token_a_amount)?;
        make_offer::save_offer(ctx, id, token_b_amount)
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        take_offer::send_tokens_to_maker(&ctx)?;
        take_offer::release_tokens_to_taker_and_close(&ctx)
    }
}
