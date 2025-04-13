use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::Offer;

use super::transfer_token;

#[derive(Accounts)]
#[instruction(/* maker: Pubkey, */ id: u64)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,

    pub token_a: InterfaceAccount<'info, Mint>, // Mint = account del token_program
    pub token_b: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed, // porque puede que no tenga account para este token
        payer = taker,
        associated_token::mint = token_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_token_account_a: Box<InterfaceAccount<'info, TokenAccount>>, // TokenAccount = account que representa el balance de x usuario de x token
    #[account(
        mut,
        associated_token::mint  = token_b,
        associated_token:: authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = token_a,
        has_one = token_b,
        seeds = [maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump = offer.bump
    )]
    pub offer: Account<'info, Offer>,
    #[account(
        mut,
        associated_token::mint = token_a,
        associated_token::authority = offer, // porque la account es manejada por el accout offer
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>, // programa (smart contract) del spl token
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn send_tokens_to_maker(ctx: &Context<TakeOffer>) -> Result<()> {
    let acc = &ctx.accounts;

    // token_b = wanted to maker
    transfer_token(
        &acc.taker_token_account_b,
        &acc.maker_token_account_b,
        acc.offer.amount_token_b,
        &acc.token_b,
        &acc.taker,
        &acc.token_program,
    )
}

pub fn release_tokens_to_taker_and_close(ctx: &Context<TakeOffer>) -> Result<()> {
    // transfer from pda (vault offer account)

    let acc = &ctx.accounts;

    let seeds = &[
        // b"offer",
        acc.maker.key.as_ref(),
        &acc.offer.id.to_le_bytes()[..],
        &[acc.offer.bump],
    ];

    let signer_seeds = [&seeds[..]];

    let accounts = TransferChecked {
        from: acc.vault.to_account_info(),
        to: acc.taker_token_account_a.to_account_info(),
        mint: acc.token_a.to_account_info(),
        authority: acc.offer.to_account_info(),
    };

    let cpi_context =
        CpiContext::new_with_signer(acc.token_program.to_account_info(), accounts, &signer_seeds);

    transfer_checked(cpi_context, acc.vault.amount, acc.token_a.decimals)?;

    // close vault (and return lamports to maker)
    let accounts = CloseAccount {
        account: acc.vault.to_account_info(),
        authority: acc.offer.to_account_info(),
        destination: acc.taker.to_account_info(),
    };

    let cpi_context =
        CpiContext::new_with_signer(acc.token_program.to_account_info(), accounts, &signer_seeds);

    close_account(cpi_context)
}
