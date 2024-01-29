use anchor_lang::prelude::*;

use anchor_lang::solana_program;
use anchor_spl::metadata::verify_sized_collection_item;
use anchor_spl::metadata::VerifySizedCollectionItem;
use anchor_spl::metadata::{
    MetadataAccount,
    VerifyCollection,
    verify_collection,
};
use anchor_spl::{
    token::Mint, 
    metadata::Metadata, 
};
use mpl_token_metadata::instructions::VerifyCollectionCpi;
use mpl_token_metadata::instructions::VerifyCollectionCpiAccounts;
use mpl_token_metadata::instructions::{
    VerifyCollectionV1Cpi, 
    VerifyCollectionV1CpiAccounts,
};
pub use solana_program::sysvar::instructions::ID as INSTRUCTIONS_ID;
pub use solana_program::sysvar::rent::ID as RENT_ID;

#[derive(Accounts)]
pub struct VerifyCollectionMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // The payer of the transaction
    /// CHECK: no need to check this as the metaplex program will do it for us
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>, // The metadata account that contains the NFT's metadata
    #[account(mut)]
    pub mint: Account<'info, Mint>, // The mint account that contains the NFT's mint
    /// CHECK: This is not dangerous as it is only the mint authority that is being passed in
    #[account(
        mut,
        seeds = [b"authority"],
        bump,
    )]
    pub mint_authority: UncheckedAccount<'info>, // The mint authority of the NFT's mint
    #[account(mut)]
    pub collection_mint: Account<'info, Mint>,
    #[account(mut)]
    pub collection_metadata: Account<'info, MetadataAccount>,
    /// CHECK: no need to check this as the metaplex program will do it for us
    #[account(mut)]
    pub collection_master_edition: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>, // The system program
    #[account(address = INSTRUCTIONS_ID)]
    /// CHECK: no need to check this
    pub sysvar_instruction: UncheckedAccount<'info>, // The sysvar instruction account
    pub token_metadata_program: Program<'info, Metadata>, // The token metadata program
}

impl<'info> VerifyCollectionMint<'info> {
    pub fn verify_collection(&mut self, bumps: &VerifyCollectionMintBumps) -> Result<()> {
        let metadata = &self.metadata.to_account_info();
        let authority = &self.mint_authority.to_account_info();
        let collection_mint = &self.collection_mint.to_account_info();
        let collection_metadata = &self.collection_metadata.to_account_info();
        let collection_master_edition = &self.collection_master_edition.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let sysvar_instructions = &self.sysvar_instruction.to_account_info();
        let spl_metadata_program = &self.token_metadata_program.to_account_info();

        let seeds = &[
            &b"authority"[..], 
            &[bumps.mint_authority]
        ];
        let signer_seeds = &[&seeds[..]];

        msg!("Mint authority: {:?}", authority.key());

        let verify_collection = VerifyCollectionV1Cpi::new(
            spl_metadata_program,
        VerifyCollectionV1CpiAccounts {
            authority,
            delegate_record: None,
            metadata,
            collection_mint,
            collection_metadata: Some(collection_metadata),
            collection_master_edition: Some(collection_master_edition),
            system_program,
            sysvar_instructions,
        });
        verify_collection.invoke_signed(signer_seeds)?;

        msg!("Collection Verified!");
        
        Ok(())
    }

    pub fn verify_collection1(&mut self, bumps: &VerifyCollectionMintBumps) -> Result<()> {
        let seeds = &[
            &b"authority"[..], 
            &[bumps.mint_authority]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_metadata_program.to_account_info();
        let cpi_accounts = VerifySizedCollectionItem {
            payer: self.payer.to_account_info(),
            metadata: self.metadata.to_account_info(),
            collection_authority: self.mint_authority.to_account_info(),
            collection_mint: self.collection_mint.to_account_info(),
            collection_metadata: self.collection_metadata.to_account_info(),
            collection_master_edition: self.collection_master_edition.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        verify_sized_collection_item(cpi_ctx, None)?;
        
        Ok(())
    }
}