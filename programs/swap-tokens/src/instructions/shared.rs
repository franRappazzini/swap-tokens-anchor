use anchor_lang::{
    prelude::{Interface, InterfaceAccount, Signer},
    Result,
};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

pub fn transfer_token<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    Ok(())
}
