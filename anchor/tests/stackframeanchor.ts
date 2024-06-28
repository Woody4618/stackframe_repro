import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stackframeanchor } from "../target/types/stackframeanchor";
import { Keypair } from "@solana/web3.js";

describe("stackframeanchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .Stackframeanchor as Program<Stackframeanchor>;
  const provider = anchor.AnchorProvider.env();
  const payer = provider.wallet as anchor.Wallet;
  const counterKeypair = Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        payer: payer.publicKey,
        counter: counterKeypair.publicKey,
      })
      .signers([counterKeypair])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
