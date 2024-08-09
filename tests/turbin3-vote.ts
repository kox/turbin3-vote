import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Turbin3Vote } from "../target/types/turbin3_vote";
import { assert } from "chai";

describe("turbin3-vote", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Turbin3Vote as Program<Turbin3Vote>;

  const uri = "https://wba.dev";

  const voteAccount = anchor.web3.PublicKey.findProgramAddressSync([
    Buffer.from(uri)
  ], program.programId)[0];

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(uri)
      .accountsPartial({
        payer: provider.wallet.publicKey,
        voteAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    let voteState = await program.account.voteState.fetch(voteAccount);
    console.log("\nYour vote score is", voteState.score.toString());

    assert.equal(voteState.score.toNumber(), 0);
  });

  it("Upvote!", async () => {
    const tx = await program.methods
      .upvote(uri)
      .accounts({
        voteAccount
      })
      .rpc();

    console.log("Your transaction signature", tx);

    let voteState = await program.account.voteState.fetch(voteAccount);
    console.log("\nYour vote score is", voteState.score.toString());

    assert.equal(voteState.score.toNumber(), 1);
  });

  it("Downvote!", async () => {
    const tx = await program.methods
      .downvote(uri)
      .accounts({
        voteAccount
      })
      .rpc();

    console.log("Your transaction signature", tx);

    let voteState = await program.account.voteState.fetch(voteAccount);
    console.log("\nYour vote score is", voteState.score.toString());

    assert.equal(voteState.score.toNumber(), 0);
  });
});
