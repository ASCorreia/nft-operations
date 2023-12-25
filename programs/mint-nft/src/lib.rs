use anchor_lang::prelude::*;

declare_id!("DxnbJwhqP2mZ7fwZipDjf4mwDETS13urV1D8dzVYWDbw");

pub mod contexts;

pub use contexts::*;

#[program]
pub mod mint_nft {

    use super::*;

    pub fn initialize(ctx: Context<MintNFT>) -> Result<()> {

        ctx.accounts.mint_nft(&ctx.bumps)
    
    }
}
