use anchor_lang::prelude::*;

declare_id!("C3dsiPjuHb9YHPMe6RjJAFzXCFZtvLV3tFbUZycea1qz");

pub mod contexts;

pub use contexts::*;

#[program]
pub mod mint_nft {

    use super::*;

    pub fn mint_nft(ctx: Context<MintNFT>) -> Result<()> {

        ctx.accounts.mint_nft(&ctx.bumps)
    
    }
}
