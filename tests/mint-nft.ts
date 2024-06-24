import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MintNft } from "../target/types/mint_nft";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { Keypair, PublicKey, SYSVAR_INSTRUCTIONS_PUBKEY, SystemProgram } from "@solana/web3.js";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("mint-nft", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as NodeWallet

  const program = anchor.workspace.MintNft as Program<MintNft>;

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

  const mintAuthority = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("authority")], program.programId)[0];

  let collectionMint: PublicKey;
  let mint: PublicKey;

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

  it("Create Collection NFT", async() => {
    const mintKeypair = Keypair.generate();
    collectionMint = await createMint(provider.connection, wallet.payer, mintAuthority, mintAuthority, 0);
    console.log("Collection Mint Key: ", collectionMint.toBase58());

    const metadata = await getMetadata(collectionMint);
    console.log("Collection Metadata Account: ", metadata.toBase58());

    const masterEdition = await getMasterEdition(collectionMint);
    console.log("Master Edition Account: ", masterEdition.toBase58());

    const destination = getAssociatedTokenAddressSync(collectionMint, wallet.publicKey);
    console.log("Destination ATA = ", destination.toBase58());

    const tx = await program.methods.createCollection().accountsPartial({
      user: wallet.publicKey,
      mint: collectionMint,
      mintAuthority,
      metadata,
      masterEdition,
      destination,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      sysvarInstruction: SYSVAR_INSTRUCTIONS_PUBKEY,
    })
    .rpc({
      skipPreflight: true,
    });
    console.log("Collection NFT minted: TxID - ", tx);
  })

  it("Mint NFT", async () => {
    // Add your test here.
    mint = await createMint(provider.connection, wallet.payer, mintAuthority, mintAuthority, 0);
    console.log("Mint", mint.toBase58());

    const metadata = await getMetadata(mint);
    console.log("Metadata", metadata.toBase58());

    const masterEdition = await getMasterEdition(mint);
    console.log("Master Edition", masterEdition.toBase58());

    const destination = getAssociatedTokenAddressSync(mint, wallet.publicKey);
    console.log("Destination", destination.toBase58());
    
    const tx = await program.methods.mintNft()
    .accountsPartial({
      owner: wallet.publicKey,
      destination,
      metadata,
      masterEdition,
      mint,
      mintAuthority,
      collectionMint,
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

  it("Verify Collection", async() => {
    const mintMetadata = await getMetadata(mint);
    console.log("Mint Metadata", mintMetadata.toBase58());

    const collectionMetadata = await getMetadata(collectionMint);
    console.log("Collection Metadata", collectionMetadata.toBase58());

    const collectionMasterEdition = await getMasterEdition(collectionMint);
    console.log("Collection Master Edition", collectionMasterEdition.toBase58());

    const tx = await program.methods.verifyCollection().accountsPartial({
      payer: wallet.publicKey,
      metadata: mintMetadata,
      mint,
      mintAuthority,
      collectionMint,
      collectionMetadata,
      collectionMasterEdition,
      systemProgram: SystemProgram.programId,
      sysvarInstruction: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    })
    .rpc({
      skipPreflight: true,
    });
    console.log("Collection Verified! Your transaction signature", tx);
  })

  xit("Verify Collection 1", async() => {
    const mintMetadata = await getMetadata(mint);
    console.log("Mint Metadata", mintMetadata.toBase58());

    const collectionMetadata = await getMetadata(collectionMint);
    console.log("Collection Metadata", collectionMetadata.toBase58());

    const collectionMasterEdition = await getMasterEdition(collectionMint);
    console.log("Collection Master Edition", collectionMasterEdition.toBase58());

    const tx = await program.methods.verifyCollection1().accountsPartial({
      payer: wallet.publicKey,
      metadata: mintMetadata,
      mint,
      mintAuthority,
      collectionMint,
      collectionMetadata,
      collectionMasterEdition,
      systemProgram: SystemProgram.programId,
      sysvarInstruction: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    })
    .rpc({
      skipPreflight: true,
    });
    console.log("Collection Verified! Your transaction signature", tx);
  })
});

