use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Offer, ANCHOR_DISCRIMINATOR};

use super::transfer_token;

#[derive(Accounts)]
#[instruction(offer_id: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mint::token_program = token_program)]
    // mint::token_program = token_program => Esta cuenta Mint debe haber sido creada por el token_program pasado en los parámetros de la instrucción
    pub token_a: InterfaceAccount<'info, Mint>, // version mas flexible de Account<'info, Mint>
    // InterfaceAccount<'info, Mint> => Acepta cualquier cuenta que implemente la interfaz compatible (TokenAccount o Mint). Muy útil para tokens que usan versiones extendidas del SPL
    #[account(mint::token_program = token_program)]
    pub token_b: InterfaceAccount<'info, Mint>, // version mas flexible de Account<'info, Mint>
    #[account(
        mut,
        associated_token::mint = token_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    // todos los decoradores de este account => Esta cuenta debe ser la cuenta ATA (Associated Token Account) para el maker, del token token_a, y debe haber sido creada por associated_token_program
    /*  asegura que:
     * el token sea el correcto (mint = token_a)
     * la cuenta sea del dueño esperado (authority = maker)
     * sea una ATA válida */
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE,
        seeds = [maker.key().as_ref(), offer_id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,
    #[account(
        mut,
        associated_token::mint = token_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>, // referencia al programa que maneja los tokens => se utiliza para que puedas llamar a instrucciones como transfer, mint_to, burn, etc., de forma genérica, sin acoplarte al spl_token original
    pub associated_token_program: Program<'info, AssociatedToken>, // programa oficial que gestiona las cuentas ATA
}

pub fn send_tokens_to_vault(ctx: &Context<MakeOffer>, token_a_amount: u64) -> Result<()> {
    let acc = &ctx.accounts;
    transfer_token(
        &acc.maker_token_account_a,
        &acc.vault,
        token_a_amount,
        &acc.token_a,
        &acc.maker,
        &acc.token_program,
    )
}

pub fn save_offer(ctx: Context<MakeOffer>, id: u64, token_b_amount: u64) -> Result<()> {
    ctx.accounts.offer.set_inner(Offer {
        id,
        maker: ctx.accounts.maker.key(),
        token_a: ctx.accounts.token_a.key(),
        token_b: ctx.accounts.token_b.key(),
        amount_token_b: token_b_amount,
        bump: ctx.bumps.offer,
    });

    Ok(())
}
