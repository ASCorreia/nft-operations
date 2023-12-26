import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MintNft } from "../target/types/mint_nft";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { SystemProgram } from "@solana/web3.js";
import { ASSOCIATED_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";

describe("mint-nft", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as NodeWallet

  const program = anchor.workspace.MintNft as Program<MintNft>;

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

  const getMetadata = async (mint: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
    return (
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mint.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
      )
    )[0];
  };

  const getMasterEdition = async (mint: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
    return (
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mint.toBuffer(),
          Buffer.from("edition"),
        ],
        TOKEN_METADATA_PROGRAM_ID
      )
    )[0];
  };

  it("MInt NFT", async () => {
    // Add your test here.
    const mintAuthority = findProgramAddressSync([Buffer.from("authority")], program.programId)[0];

    const mint = await createMint(provider.connection, wallet.payer, mintAuthority, mintAuthority, 0);
    console.log("Mint", mint.toBase58());

    const metadata = await getMetadata(mint);
    console.log("Metadata", metadata.toBase58());

    const masterEdition = await getMasterEdition(mint);
    console.log("Master Edition", masterEdition.toBase58());

    const destination = getAssociatedTokenAddressSync(mint, wallet.publicKey);
    const tx = await program.methods.mintNft()
    .accounts({
      owner: wallet.publicKey,
      destination,
      metadata,
      masterEdition,
      mint,
      mintAuthority: mintAuthority,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      sysvarInstruction: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    })
    .rpc({
      skipPreflight: true,
    });
    console.log("\n\nNFT Minted! Your transaction signature", tx);
  });
});
