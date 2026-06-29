import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { CropBatch } from "../target/types/crop_batch";
import { assert } from "chai";

describe("crop-batch", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CropBatch as Program<CropBatch>;
  const provider = anchor.getProvider();
  const farmer = (provider.wallet as anchor.Wallet).payer;

  const batchName = "WinterHarvest2026";
  const logisticsPartner = anchor.web3.Keypair.generate();

  // Derive PDAs
  const [farmerPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("farmer"), farmer.publicKey.toBuffer()],
    program.programId
  );

  const [batchPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("batch"), farmer.publicKey.toBuffer(), Buffer.from(batchName)],
    program.programId
  );

  const [partnerStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("logistics-partner"),
      farmer.publicKey.toBuffer(),
      logisticsPartner.publicKey.toBuffer(),
    ],
    program.programId
  );

  before(async () => {
    // Airdrop SOL to the logistics partner so they can pay for transaction fees
    const signature = await provider.connection.requestAirdrop(
      logisticsPartner.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    const latestBlockhash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...latestBlockhash
    });
  });

  it("Mints a new crop batch", async () => {
    await program.methods
      .mintBatch(batchName)
      .accounts({
        signer: farmer.publicKey,
        farmer: farmerPda,
        batchAccount: batchPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([farmer])
      .rpc();

    const batchState = await program.account.batchState.fetch(batchPda);
    assert.equal(batchState.name, batchName);
    assert.equal(batchState.checkpointCount.toString(), "0");
    assert.deepEqual(batchState.status, { active: {} });
    assert.equal(batchState.authority.toBase58(), farmer.publicKey.toBase58());
  });

  it("Adds a checkpoint as the farmer (bypasses registry checks)", async () => {
    const checkpointName = "At port in lagos";

    const batchStateBefore = await program.account.batchState.fetch(batchPda);
    const checkpointIndex = batchStateBefore.checkpointCount;

    const indexBuf = Buffer.alloc(8);
    indexBuf.writeBigUInt64LE(BigInt(checkpointIndex.toString()));

    const [checkpointPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("checkpoint"),
        batchPda.toBuffer(),
        indexBuf,
      ],
      program.programId
    );

    await program.methods
      .addCheckpoint(checkpointName)
      .accounts({
        signer: farmer.publicKey,
        batchAccount: batchPda,
        checkpointAccount: checkpointPda,
        partnerState: program.programId, // pass dummy program ID since signer is authority
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([farmer])
      .rpc();

    const checkpointState = await program.account.checkpointState.fetch(checkpointPda);
    assert.equal(checkpointState.name, checkpointName);
    assert.equal(checkpointState.index.toString(), checkpointIndex.toString());
    assert.equal(checkpointState.batch.toBase58(), batchPda.toBase58());

    const batchStateAfter = await program.account.batchState.fetch(batchPda);
    assert.equal(batchStateAfter.checkpointCount.toString(), "1");
    const statusObj = batchStateAfter.status as any;
    assert.isDefined(statusObj.checkpoint);
    assert.equal(statusObj.checkpoint[0].toString(), "0");
  });

  it("Registers a logistics partner", async () => {
    await program.methods
      .registerLogisticsPartner(logisticsPartner.publicKey)
      .accounts({
        farmer: farmer.publicKey,
        partnerState: partnerStatePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([farmer])
      .rpc();

    const partnerState = await program.account.logisticsPartnerState.fetch(partnerStatePda);
    assert.equal(partnerState.farmer.toBase58(), farmer.publicKey.toBase58());
    assert.equal(partnerState.partner.toBase58(), logisticsPartner.publicKey.toBase58());
  });

  it("Adds a checkpoint as the authorized logistics partner", async () => {
    const checkpointName = "Le Havre Port Checkpoint";

    const batchStateBefore = await program.account.batchState.fetch(batchPda);
    const checkpointIndex = batchStateBefore.checkpointCount;

    const indexBuf = Buffer.alloc(8);
    indexBuf.writeBigUInt64LE(BigInt(checkpointIndex.toString()));

    const [checkpointPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("checkpoint"),
        batchPda.toBuffer(),
        indexBuf,
      ],
      program.programId
    );

    await program.methods
      .addCheckpoint(checkpointName)
      .accounts({
        signer: logisticsPartner.publicKey,
        batchAccount: batchPda,
        checkpointAccount: checkpointPda,
        partnerState: partnerStatePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([logisticsPartner])
      .rpc();

    const checkpointState = await program.account.checkpointState.fetch(checkpointPda);
    assert.equal(checkpointState.name, checkpointName);
    assert.equal(checkpointState.authority.toBase58(), logisticsPartner.publicKey.toBase58());
    assert.equal(checkpointState.index.toString(), checkpointIndex.toString());
  });

  it("Fails to add a checkpoint as an unauthorized partner", async () => {
    const randomPartner = anchor.web3.Keypair.generate();

    // Airdrop SOL to the random partner so they can pay for account creation rent
    const signature = await provider.connection.requestAirdrop(
      randomPartner.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    const latestBlockhash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...latestBlockhash
    });

    const checkpointName = "Fake Terminal";

    const batchStateBefore = await program.account.batchState.fetch(batchPda);
    const checkpointIndex = batchStateBefore.checkpointCount;

    const indexBuf = Buffer.alloc(8);
    indexBuf.writeBigUInt64LE(BigInt(checkpointIndex.toString()));

    const [checkpointPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("checkpoint"),
        batchPda.toBuffer(),
        indexBuf,
      ],
      program.programId
    );

    try {
      await program.methods
        .addCheckpoint(checkpointName)
        .accounts({
          signer: randomPartner.publicKey,
          batchAccount: batchPda,
          checkpointAccount: checkpointPda,
          partnerState: program.programId, // pass dummy
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([randomPartner])
        .rpc();
      assert.fail("Should have failed checkpoint creation");
    } catch (err) {
      assert.include(err.message, "Unauthorized");
    }
  });

  it("Fails invalid status transitions", async () => {
    try {
      // Trying to transition checkpoint -> active (which is invalid)
      await program.methods
        .updateStatus({ active: {} })
        .accounts({
          authority: farmer.publicKey,
          batchAccount: batchPda,
        } as any)
        .signers([farmer])
        .rpc();
      assert.fail("Should have failed transitioning back to Active");
    } catch (err) {
      assert.include(err.message, "InvalidStateTransition");
    }
  });

  it("Transitions status to InTransit", async () => {
    await program.methods
      .updateStatus({ inTransit: {} })
      .accounts({
        authority: farmer.publicKey,
        batchAccount: batchPda,
      } as any)
      .signers([farmer])
      .rpc();

    const batchState = await program.account.batchState.fetch(batchPda);
    assert.deepEqual(batchState.status, { inTransit: {} });
  });

  it("Transitions status to Sold", async () => {
    await program.methods
      .updateStatus({ sold: {} })
      .accounts({
        authority: farmer.publicKey,
        batchAccount: batchPda,
      } as any)
      .signers([farmer])
      .rpc();

    const batchState = await program.account.batchState.fetch(batchPda);
    assert.deepEqual(batchState.status, { sold: {} });
  });

  it("Deregisters the logistics partner", async () => {
    await program.methods
      .deregisterLogisticsPartner()
      .accounts({
        farmer: farmer.publicKey,
        partnerState: partnerStatePda,
      } as any)
      .signers([farmer])
      .rpc();

    const accountInfo = await provider.connection.getAccountInfo(partnerStatePda);
    assert.isNull(accountInfo);
  });

  it("Closes the batch account", async () => {
    const balanceBefore = await provider.connection.getBalance(farmer.publicKey);

    await program.methods
      .closeBatch()
      .accounts({
        farmer: farmer.publicKey,
        batchAccount: batchPda,
      })
      .signers([farmer])
      .rpc();

    const balanceAfter = await provider.connection.getBalance(farmer.publicKey);
    assert.isTrue(balanceAfter > balanceBefore - 100000); // balance increases (reclaimed rent minus fee)

    // Check that account no longer exists
    const batchAccountInfo = await provider.connection.getAccountInfo(batchPda);
    assert.isNull(batchAccountInfo);
  });
});
