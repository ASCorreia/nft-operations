use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, metadata::{MasterEditionAccount, Metadata, MetadataAccount}, token::{
        mint_to, 
        Mint, 
        MintTo, 
        Token, 
        TokenAccount,
    }
};
use anchor_spl::metadata::mpl_token_metadata::{
    instructions::{
        CreateMasterEditionV3Cpi, 
        CreateMasterEditionV3CpiAccounts, 
        CreateMasterEditionV3InstructionArgs, 
        CreateMetadataAccountV3Cpi, 
        CreateMetadataAccountV3CpiAccounts, 
        CreateMetadataAccountV3InstructionArgs
    }, 
    types::{
        CollectionDetails, 
        Creator, 
        DataV2
    }
};
pub use anchor_lang::solana_program::sysvar::instructions::ID as INSTRUCTIONS_ID;

#[derive(Accounts)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        mut,
        mint::decimals = 0,
        mint:: authority = mint_authority,
    )]
    mint: Account<'info, Mint>,
    /// CHECK: This is not dangerous as it is only the mint authority that is being passed in
    #[account(
        mut,
        seeds = [b"authority"],
        bump,
    )]
    pub mint_authority: UncheckedAccount<'info>, // The mint authority of the NFT's Collection
    /// CHECK: This is safe and will be checked by metaplex program
    #[account(mut)]
    metadata: Account<'info, MetadataAccount>,
    /// CHECK: This is safe and will be checked by metaplex program
    #[account(mut)]
    master_edition: Account<'info, MasterEditionAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    destination: Account<'info, TokenAccount>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_metadata_program: Program<'info, Metadata>,
    #[account(address = INSTRUCTIONS_ID)]
    /// CHECK: no need to check this
    pub sysvar_instruction: UncheckedAccount<'info>, // The sysvar instruction account
}

impl<'info> CreateCollection<'info> {
    pub fn create_collection(&mut self, bumps: &CreateCollectionBumps) -> Result<()> {

        let metadata = &self.metadata.to_account_info();
        let master_edition = &self.master_edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let authority = &self.mint_authority.to_account_info();
        let payer = &self.user.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let spl_token_program = &self.token_program.to_account_info();
        let spl_metadata_program = &self.token_metadata_program.to_account_info();

        let seeds = &[
            &b"authority"[..], 
            &[bumps.mint_authority]
        ];
        let signer_seeds = &[&seeds[..]];

        let creator = vec![
            Creator {
                address: self.mint_authority.key().clone(),
                verified: true,
                share: 100,
            },
        ];
        
        let metadata_account = CreateMetadataAccountV3Cpi::new(
            spl_metadata_program, 
            CreateMetadataAccountV3CpiAccounts {
                metadata,
                mint,
                mint_authority: authority,
                payer,
                update_authority: (authority, true),
                system_program,
                rent: None,
            },
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name: "DummyCollection".to_owned(),
                    symbol: "DC".to_owned(),
                    uri: "".to_owned(),
                    seller_fee_basis_points: 0,
                    creators: Some(creator),
                    collection: None,
                    uses: None,
                },
                is_mutable: true,
                collection_details: Some(
                    CollectionDetails::V1 { 
                        size: 0 
                    }
                )
            }
        );
        metadata_account.invoke_signed(signer_seeds)?;
        msg!("Metadata Account created!");

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.destination.to_account_info(),
            authority: self.mint_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        mint_to(cpi_ctx, 1)?;
        msg!("Collection NFT minted!");

        let master_edition_account = CreateMasterEditionV3Cpi::new(
            spl_metadata_program,
            CreateMasterEditionV3CpiAccounts {
                edition: master_edition,
                update_authority: authority,
                mint_authority: authority,
                mint,
                payer,
                metadata,
                token_program: spl_token_program,
                system_program,
                rent: None,
            },
            CreateMasterEditionV3InstructionArgs {
                max_supply: Some(0),
            }
        );
        master_edition_account.invoke_signed(signer_seeds)?;
        msg!("Master Edition Account created");
        
        Ok(())
    }
}