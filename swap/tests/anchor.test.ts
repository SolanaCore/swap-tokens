import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { Swap } from "../target/types/swap";
import {
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  getAssociatedTokenAddress,
  mintTo,
} from "@solana/spl-token";
import { BN } from "bn.js";
import * as assert from "assert";

describe("Swap Program - createOffer", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Swap as Program<Swap>;
  it("should create an offer using inferred seeds", async () => {
    const connection = provider.connection;
    const proposer = provider.wallet;
    const offerId = new BN(100080); // unique offer id (u64)

    // Airdrop some SOL
    await connection.requestAirdrop(
      proposer.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );

    // Create two test mints
    const tokenMint0 = await createMint(
      connection,
      proposer.payer,
      proposer.publicKey,
      null,
      9
    );
    const tokenMint1 = await createMint(
      connection,
      proposer.payer,
      proposer.publicKey,
      null,
      9
    );

    // Create or get token accounts for proposer
    const proposerToken0 = await getOrCreateAssociatedTokenAccount(
      connection,
      proposer.payer,
      tokenMint0,
      proposer.publicKey
    );

    // Mint token_0 to proposer
    await mintTo(
      connection,
      proposer.payer,
      tokenMint0,
      proposerToken0.address,
      proposer.payer,
      1_000_000_000
    );

    // Derive the Offer PDA using `offerId`
    const [offerPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("offer"),
        proposer.publicKey.toBuffer(),
        offerId.toArrayLike(Buffer, "le", 8), // u64 as 8-byte LE
      ],
      program.programId
    );

    // Vaults (owned by the offer PDA)
    const vault0 = await getAssociatedTokenAddress(
      tokenMint0,
      offerPda,
      true
    );
    const vault1 = await getAssociatedTokenAddress(
      tokenMint1,
      offerPda,
      true
    );

    // Invoke the method
    const tx = await program.methods
      .createOffer(offerId, new BN(10000), new BN(50))
      .accounts({
        proposer: proposer.publicKey,
        offer: offerPda,
        token0: proposerToken0.address,
        token0Mint: tokenMint0,
        token1Mint: tokenMint1,
        vault0,
        vault1,
        systemProgram: SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      })
      .signers([proposer.payer])
      .rpc();

    console.log("✅ Transaction:", tx);

    const offer = await program.account.offer.fetch(offerPda);
    console.log(offer);
  });
});