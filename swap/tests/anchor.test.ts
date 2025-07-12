import { randomBytes } from "node:crypto";
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
  mintTo,
  getAccount,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import { BN } from "bn.js";
import * as assert from "assert";

describe("Swap Program - createOffer", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Swap as Program<Swap>;

  it("should create an offer with correct seeds and transfer tokens to vaults", async () => {
    const connection = provider.connection;
    const proposer = provider.wallet;
    const offerId = new BN("1234567890");

    await connection.requestAirdrop(proposer.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);

    const tokenMint0 = await createMint(connection, proposer.payer, proposer.publicKey, null, 9);
    const tokenMint1 = await createMint(connection, proposer.payer, proposer.publicKey, null, 9);

    const proposerToken0 = (
      await getOrCreateAssociatedTokenAccount(connection, proposer.payer, tokenMint0, proposer.publicKey)
    ).address;

    await mintTo(connection, proposer.payer, tokenMint0, proposerToken0, proposer.payer, 1_000_000_000);

    const [offerPda, offerBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("offer"),
        proposer.publicKey.toBuffer(),
        offerId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    const vault0Ata = await getAssociatedTokenAddress(tokenMint0, offerPda, true);
    const vault1Ata = await getAssociatedTokenAddress(tokenMint1, offerPda, true);

    // Debug info
    console.log("🔑 proposer:", proposer.publicKey.toBase58());
    console.log("📦 offer PDA:", offerPda.toBase58());
    console.log("🪙 tokenMint0:", tokenMint0.toBase58());
    console.log("🪙 tokenMint1:", tokenMint1.toBase58());
    console.log("🎯 proposerToken0 ATA:", proposerToken0.toBase58());
    console.log("🏦 vault0 ATA:", vault0Ata.toBase58());
    console.log("🏦 vault1 ATA:", vault1Ata.toBase58());
    console.log("🆔 offerId:", offerId.toString());

    try {
      await program.methods
        .createOffer(new BN(100), new BN(50), offerId)
        .accounts({
          proposer: proposer.publicKey,
          offer: offerPda,
          token0: proposerToken0,
          token0Mint: tokenMint0,
          token1Mint: tokenMint1,
          vault0: vault0Ata,
          vault1: vault1Ata,
          systemProgram: SystemProgram.programId,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        })
        .signers([proposer.payer])
        .rpc();

      const offer = await program.account.offer.fetch(offerPda);
      console.log("✅ Offer account data:", offer);

      // Validate Offer state
      assert.strictEqual(offer.proposer.toBase58(), proposer.publicKey.toBase58());
      assert.ok(offer.token0Amount.eq(new BN(100)));
      assert.ok(offer.token1Amount.eq(new BN(50)));
      assert.strictEqual(offer.token0Mint.toBase58(), tokenMint0.toBase58());
      assert.strictEqual(offer.token1Mint.toBase58(), tokenMint1.toBase58());
      assert.ok(offer.offerId.eq(offerId));
      assert.ok(offer.isActive);
      assert.ok(!offer.isFulfilled);
      assert.ok(!offer.isEdited);

      const vault0Info = await getAccount(connection, vault0Ata);
      assert.strictEqual(vault0Info.owner.toBase58(), offerPda.toBase58());

      const proposerTokenInfo = await getAccount(connection, proposerToken0);
    } catch (err) {
      console.error("❌ Error:", err);
      throw err;
    }
  });
});
