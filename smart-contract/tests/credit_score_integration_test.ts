import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { CreditScore } from "../target/types/credit_score";
import { assert } from "chai";

describe("credit-score-integration", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CreditScore as Program<CreditScore>;
  const cropBatchProgram = anchor.workspace.CropBatch;
  const provider = anchor.getProvider();
  const farmer = anchor.web3.Keypair.generate();
  const tradeAuthority = anchor.web3.Keypair.generate();

  // Derive PDAs
  const [creditAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("credit-account"), farmer.publicKey.toBuffer()],
    program.programId
  );

  const [creditConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  const [batchPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("batch"), farmer.publicKey.toBuffer(), Buffer.from("TestBatch")],
    cropBatchProgram.programId
  );

  const [farmerPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("farmer"), farmer.publicKey.toBuffer()],
    cropBatchProgram.programId
  );

  let tradeEscrowId: anchor.web3.PublicKey;

  before(async () => {
    // Airdrop SOL to farmer and tradeAuthority
    const sig1 = await provider.connection.requestAirdrop(farmer.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
    const latestBlockhash1 = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({ signature: sig1, ...latestBlockhash1 });

    const sig2 = await provider.connection.requestAirdrop(tradeAuthority.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    const latestBlockhash2 = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({ signature: sig2, ...latestBlockhash2 });

    // Determine trade_escrow program ID
    tradeEscrowId = anchor.workspace.TradeEscrow 
      ? anchor.workspace.TradeEscrow.programId 
      : anchor.web3.Keypair.generate().publicKey;

    // Initialize config if not already initialized
    const configInfo = await provider.connection.getAccountInfo(creditConfigPda);
    if (configInfo === null) {
      await program.methods
        .initializeConfig(provider.wallet.publicKey, tradeEscrowId)
        .accounts({
          deployer: provider.wallet.publicKey,
          config: creditConfigPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();
    }

    // Initialize credit account first via initializeCredit
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

    // Mint crop batch using CropBatch program so we have a valid batch account for checks
    await cropBatchProgram.methods
      .mintBatch("TestBatch")
      .accounts({
        signer: farmer.publicKey,
        farmer: farmerPda,
        batchAccount: batchPda,
        creditAccount: creditAccountPda,
        creditScoreProgram: program.programId,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([farmer])
      .rpc();
  });

  it("Initializes credit account directly", async () => {
    const creditState = await program.account.creditAccount.fetch(creditAccountPda);
    assert.equal(creditState.farmer.toBase58(), farmer.publicKey.toBase58());
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
        authority: farmer.publicKey,
        config: creditConfigPda,
        tradeEscrowProgram: tradeEscrowId,
        farmer: farmer.publicKey,
        batchAccount: batchPda,
        creditAccount: creditAccountPda,
      } as any)
      .signers([farmer])
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

  it("Fails to update score if signer is not the farmer (Unauthorized)", async () => {
    try {
      await program.methods
        .updateScore(new anchor.BN(10000))
        .accounts({
          authority: tradeAuthority.publicKey,
          config: creditConfigPda,
          tradeEscrowProgram: tradeEscrowId,
          farmer: farmer.publicKey,
          batchAccount: batchPda,
          creditAccount: creditAccountPda,
        } as any)
        .signers([tradeAuthority])
        .rpc();
      assert.fail("Should have failed with Unauthorized error");
    } catch (err: any) {
      assert.include(err.message, "Unauthorized");
    }
  });

  it("Updates score above threshold (Eligible)", async () => {
    // Increment score: base 10 + bonus (30000 / 1000) = 40
    // New score: 25 + 40 = 65 (>= 50)
    await program.methods
      .updateScore(new anchor.BN(30000))
      .accounts({
        authority: farmer.publicKey,
        config: creditConfigPda,
        tradeEscrowProgram: tradeEscrowId,
        farmer: farmer.publicKey,
        batchAccount: batchPda,
        creditAccount: creditAccountPda,
      } as any)
      .signers([farmer])
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
