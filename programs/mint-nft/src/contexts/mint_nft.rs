use anchor_lang::{prelude::*, solana_program};
use anchor_spl::{
    token::{TokenAccount, Mint, Token}, 
    metadata::{MetadataAccount, MasterEditionAccount, Metadata}, 
    associated_token::AssociatedToken
};
use mpl_token_metadata::{
    instructions::{MintCpi, MintCpiAccounts, MintInstructionArgs, CreateMetadataAccountV3Cpi, CreateMetadataAccountV3CpiAccounts, CreateMetadataAccountV3InstructionArgs, CreateMasterEditionV3Cpi, CreateMasterEditionV3InstructionArgs, CreateMasterEditionV3CpiAccounts}, 
    types::{MintArgs, DataV2, Creator}
};
pub use solana_program::sysvar::instructions::ID as INSTRUCTIONS_ID;
pub use solana_program::sysvar::rent::ID as RENT_ID;

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub owner: Signer<'info>, // The owner of the account that should receive the NFT
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub destination: Account<'info, TokenAccount>, // The account to which the NFT should be sent
    /// CHECK: no need to check this as the metaplex program will do it for us
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>, // The metadata account that contains the NFT's metadata
    /// CHECK: no need to check this as the metaplex program will do it for us
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>, // The master edition account that contains the NFT's master edition
    #[account(mut)]
    pub mint: Account<'info, Mint>, // The mint account that contains the NFT's mint
    /// CHECK: This is not dangerous as it is only the mint authority that is being passed in
    #[account(
        mut,
        seeds = [b"authority"],
        bump,
    )]
    pub mint_authority: UncheckedAccount<'info>, // The mint authority of the NFT's mint
    pub system_program: Program<'info, System>, // The system program
    pub token_program: Program<'info, Token>, // The token program
    pub associated_token_program: Program<'info, AssociatedToken>, // The associated token program
    #[account(address = INSTRUCTIONS_ID)]
    /// CHECK: no need to check this
    pub sysvar_instruction: UncheckedAccount<'info>, // The sysvar instruction account
    #[account(address = RENT_ID)]
    /// CHECK: no need to check this
    pub rent: UncheckedAccount<'info>, // The sysvar instruction account
    pub token_metadata_program: Program<'info, Metadata>, // The token metadata program
}

impl<'info> MintNFT<'info> {
    pub fn mint_nft(&mut self, bumps: &MintNFTBumps) -> Result<()> {

        let token = &self.destination.to_account_info();
        let token_owner = &self.owner.to_account_info();
        let metadata = &self.metadata.to_account_info();
        let master_edition = &self.master_edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let authority = &self.mint_authority.to_account_info();
        let payer = &self.owner.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let sysvar_instructions = &self.sysvar_instruction.to_account_info();
        let spl_token_program = &self.token_program.to_account_info();
        let spl_ata_program = &self.associated_token_program.to_account_info();
        let spl_metadata_program = &self.token_metadata_program.to_account_info();
        let rent = &self.rent.to_account_info();

        let seeds = &[
            &b"authority"[..], 
            &[bumps.mint_authority]
        ];
        let signer_seeds = &[&seeds[..]];

        let creator = vec![
            Creator {
                address: self.mint_authority.key(),
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
                rent: Some(rent),
            }, 
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name: "Mint Test".to_string(),
                    symbol: "YAY".to_string(),
                    uri: "https://arweave.net/Pe4erqz3MZoywHqntUGZoKIoH0k9QUykVDFVMjpJ08s".to_string(),
                    seller_fee_basis_points: 0,
                    creators: Some(creator),
                    collection: None,
                    uses: None
                },
                is_mutable: true,
                collection_details: None,
            }
        );
        metadata_account.invoke_signed(signer_seeds)?;

        let mint_cpi = MintCpi::new(
            spl_metadata_program,
            MintCpiAccounts{
                token,
                token_owner: Some(token_owner),
                metadata,
                master_edition: Some(master_edition),
                token_record: None,
                mint,
                authority,
                delegate_record: None,
                payer,
                system_program,
                sysvar_instructions,
                spl_token_program,
                spl_ata_program,
                authorization_rules: None,
                authorization_rules_program: None,
            },
            MintInstructionArgs {
                mint_args: MintArgs::V1 {
                    amount: 1,
                    authorization_data: None,
                }
            }
        );
        mint_cpi.invoke_signed(signer_seeds)?;

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
                rent: Some(rent),
            },
            CreateMasterEditionV3InstructionArgs {
                max_supply: Some(0),
            }
        );
        master_edition_account.invoke_signed(signer_seeds)?;

        Ok(())
        
    }
}