use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("zkVau1tProgram111111111111111111111111111");

#[program]
pub mod zkvault_program {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = ctx.accounts.owner.key();
        vault.bump = ctx.bumps.vault;
        Ok(())
    }

    pub fn deposit_with_proof(ctx: Context<DepositWithProof>, data: Vec<u8>, proof: Vec<u8>) -> Result<()> {
        // Verify ZK proof here (placeholder for actual verification)
        // For production, integrate with Noir verifier

        let vault = &mut ctx.accounts.vault;
        vault.data = data;
        vault.proof = proof;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Vault::LEN,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositWithProof<'info> {
    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub data: Vec<u8>,
    pub proof: Vec<u8>,
    pub bump: u8,
}

impl Vault {
    const LEN: usize = 32 + 4 + 1000 + 4 + 1000 + 1; // Adjust sizes as needed
}