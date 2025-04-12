use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,
    pub maker: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub amount_token_b: u64,
    pub bump: u8, // ?
}
