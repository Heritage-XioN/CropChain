import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { CreditScore } from "../target/types/credit_score";
import { assert } from "chai";

describe("credit-score-integration", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CreditScore as Program<CreditScore>;
  const provider = anchor.getProvider();
  const farmer = anchor.web3.Keypair.generate();
  const tradeAuthority = anchor.web3.Keypair.generate();

  // Derive PDA
  const [creditAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("credit-account"), farmer.publicKey.toBuffer()],
    program.programId
  );

  before(async () => {
    // Airdrop SOL to farmer and tradeAuthority
    const sig1 = await provider.connection.requestAirdrop(farmer.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    const latestBlockhash1 = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({ signature: sig1, ...latestBlockhash1 });

    const sig2 = await provider.connection.requestAirdrop(tradeAuthority.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    const latestBlockhash2 = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({ signature: sig2, ...latestBlockhash2 });
  });

  it("Initializes credit account directly", async () => {
    await program.methods
      .initializeCredit()
      .accounts({
        signer: farmer.publicKey,
        farmer: farmer.publicKey,
        creditAccount: creditAccountPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([farmer])
      .rpc();

    const creditState = await program.account.creditAccount.fetch(creditAccountPda);
    assert.equal(creditState.farmer.toBase58(), farmer.publicKey.toBase58());
    assert.equal(creditState.score.toString(), "0");
  });

  it("Gets score and eligibility status via view (Ineligible)", async () => {
    const profile = await program.methods
      .getScore()
      .accounts({
        creditAccount: creditAccountPda,
      } as any)
      .view();

    assert.equal(profile.score.toString(), "0");
    assert.deepEqual(profile.eligibility, { ineligible: {} });
  });

  it("Updates score (Ineligible threshold)", async () => {
    // Increment score: base 10 + bonus (15000 / 1000) = 25
    await program.methods
      .updateScore(new anchor.BN(15000))
      .accounts({
        authority: tradeAuthority.publicKey,
        farmer: farmer.publicKey,
        creditAccount: creditAccountPda,
      } as any)
      .signers([tradeAuthority])
      .rpc();

    const profile = await program.methods
      .getScore()
      .accounts({
        creditAccount: creditAccountPda,
      } as any)
      .view();

    assert.equal(profile.score.toString(), "25");
    assert.deepEqual(profile.eligibility, { ineligible: {} });
  });

  it("Updates score above threshold (Eligible)", async () => {
    // Increment score: base 10 + bonus (30000 / 1000) = 40
    // New score: 25 + 40 = 65 (>= 50)
    await program.methods
      .updateScore(new anchor.BN(30000))
      .accounts({
        authority: tradeAuthority.publicKey,
        farmer: farmer.publicKey,
        creditAccount: creditAccountPda,
      } as any)
      .signers([tradeAuthority])
      .rpc();

    const profile = await program.methods
      .getScore()
      .accounts({
        creditAccount: creditAccountPda,
      } as any)
      .view();

    assert.equal(profile.score.toString(), "65");
    assert.deepEqual(profile.eligibility, { eligible: {} });
  });
});
