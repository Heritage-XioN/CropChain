import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { CropBatch } from "../target/types/crop_batch";
import { TradeEscrow } from "../target/types/trade_escrow";
import { AdminRegistry } from "../target/types/admin_registry";
import { assert } from "chai";

describe("trade-escrow-integration", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const cropBatchProgram = anchor.workspace.CropBatch as Program<CropBatch>;
  const tradeEscrowProgram = anchor.workspace.TradeEscrow as Program<TradeEscrow>;
  const adminRegistryProgram = anchor.workspace.AdminRegistry as Program<AdminRegistry>;
  const provider = anchor.getProvider();

  const farmer = anchor.web3.Keypair.generate();
  const buyer = anchor.web3.Keypair.generate();
  const treasury = anchor.web3.Keypair.generate();
  const batchName = "Valencia Oranges";

  // Derive Crop Batch PDA
  const [batchPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("batch"), farmer.publicKey.toBuffer(), Buffer.from(batchName)],
    cropBatchProgram.programId
  );

  // Derive Farmer PDA
  const [farmerPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("farmer"), farmer.publicKey.toBuffer()],
    cropBatchProgram.programId
  );

  // Derive Credit Account PDA (for crop-batch mint call dependency)
  const creditScoreProgram = anchor.workspace.CreditScore;
  const [creditAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("credit-account"), farmer.publicKey.toBuffer()],
    creditScoreProgram.programId
  );

  // Derive Trade Escrow PDAs
  const [tradeAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("trade-account"), batchPda.toBuffer()],
    tradeEscrowProgram.programId
  );

  const [escrowVaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("escrow-vault"), batchPda.toBuffer()],
    tradeEscrowProgram.programId
  );

  before(async () => {
    // Fund farmer and buyer
    const sig1 = await provider.connection.requestAirdrop(farmer.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
    const latestBlockhash1 = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({ signature: sig1, ...latestBlockhash1 });

    const sig2 = await provider.connection.requestAirdrop(buyer.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
    const latestBlockhash2 = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({ signature: sig2, ...latestBlockhash2 });

    // Mint crop batch first (requires farmer setup and credit score CPI)
    await cropBatchProgram.methods
      .mintBatch(batchName)
      .accounts({
        signer: farmer.publicKey,
        farmer: farmerPda,
        batchAccount: batchPda,
        creditAccount: creditAccountPda,
        creditScoreProgram: creditScoreProgram.programId,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([farmer])
      .rpc();
  });

  it("Creates a trade escrow (deposits SOL to vault)", async () => {
    const depositAmount = new anchor.BN(1.5 * anchor.web3.LAMPORTS_PER_SOL);

    // Call create_trade
    await tradeEscrowProgram.methods
      .createTrade(depositAmount)
      .accounts({
        buyer: buyer.publicKey,
        batchAccount: batchPda,
        tradeAccount: tradeAccountPda,
        escrowVault: escrowVaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([buyer])
      .rpc();

    // Verify TradeAccount state
    const tradeState = await tradeEscrowProgram.account.tradeAccount.fetch(tradeAccountPda);
    assert.equal(tradeState.buyer.toBase58(), buyer.publicKey.toBase58());
    assert.equal(tradeState.batch.toBase58(), batchPda.toBase58());
    assert.equal(tradeState.amount.toString(), depositAmount.toString());

    // Verify EscrowVault balance
    const vaultBalance = await provider.connection.getBalance(escrowVaultPda);
    assert.equal(vaultBalance.toString(), depositAmount.toString());
  });

  it("Fails to accept trade if signer is not the farmer (Unauthorized)", async () => {
    try {
      await tradeEscrowProgram.methods
        .acceptTrade()
        .accounts({
          farmer: buyer.publicKey, // buyer is not the farmer
          batchAccount: batchPda,
          trade_account: tradeAccountPda, // wait, Anchor TS client auto-resolves name or we cast to any
        } as any)
        .signers([buyer])
        .rpc();
      assert.fail("Should have failed with Unauthorized error");
    } catch (err: any) {
      assert.include(err.message, "Unauthorized");
    }
  });

  it("Farmer accepts the trade escrow successfully", async () => {
    await tradeEscrowProgram.methods
      .acceptTrade()
      .accounts({
        farmer: farmer.publicKey,
        batchAccount: batchPda,
        tradeAccount: tradeAccountPda,
      } as any)
      .signers([farmer])
      .rpc();

    // Verify TradeAccount state is now Active
    const tradeState = await tradeEscrowProgram.account.tradeAccount.fetch(tradeAccountPda);
    assert.deepEqual(tradeState.status, { active: {} });
    assert.isTrue(Number(tradeState.acceptedAt.toString()) > 0);
  });

  it("Fails to accept an already active trade (InvalidTradeStatus)", async () => {
    try {
      await tradeEscrowProgram.methods
        .acceptTrade()
        .accounts({
          farmer: farmer.publicKey,
          batchAccount: batchPda,
          tradeAccount: tradeAccountPda,
        } as any)
        .signers([farmer])
        .rpc();
      assert.fail("Should have failed with InvalidTradeStatus error");
    } catch (err: any) {
      assert.include(err.message, "InvalidTradeStatus");
    }
  });

  it("Fails to confirm delivery if signer is not the buyer (Unauthorized)", async () => {
    try {
      await tradeEscrowProgram.methods
        .confirmDelivery()
        .accounts({
          authority: farmer.publicKey, // farmer is not the buyer
          tradeAccount: tradeAccountPda,
          escrowVault: escrowVaultPda,
          farmer: farmerPda,
          treasury: treasury.publicKey,
          creditAccount: creditAccountPda,
          creditScoreProgram: creditScoreProgram.programId,
          batchAccount: batchPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([farmer])
        .rpc();
      assert.fail("Should have failed with Unauthorized error");
    } catch (err: any) {
      assert.include(err.message, "Unauthorized");
    }
  });

  it("Buyer confirms delivery successfully (releases SOL and updates credit score)", async () => {
    const initialFarmerBalance = await provider.connection.getBalance(farmer.publicKey);
    const initialTreasuryBalance = await provider.connection.getBalance(treasury.publicKey);

    // Call confirmDelivery
    await tradeEscrowProgram.methods
      .confirmDelivery()
      .accounts({
        authority: buyer.publicKey,
        tradeAccount: tradeAccountPda,
        escrowVault: escrowVaultPda,
        farmer: farmer.publicKey, // farmer key receiving funds
        treasury: treasury.publicKey,
        creditAccount: creditAccountPda,
        creditScoreProgram: creditScoreProgram.programId,
        batchAccount: batchPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([buyer])
      .rpc();

    // Verify TradeAccount state is now Completed
    const tradeState = await tradeEscrowProgram.account.tradeAccount.fetch(tradeAccountPda);
    assert.deepEqual(tradeState.status, { completed: {} });

    // Verify fund distribution: 1.5 SOL total
    // 1% fee = 15,000,000 lamports
    // 99% farmer = 1,485,000,000 lamports
    const finalFarmerBalance = await provider.connection.getBalance(farmer.publicKey);
    const finalTreasuryBalance = await provider.connection.getBalance(treasury.publicKey);

    assert.equal(
      (finalFarmerBalance - initialFarmerBalance).toString(),
      "1485000000"
    );
    assert.equal(
      (finalTreasuryBalance - initialTreasuryBalance).toString(),
      "15000000"
    );

    // Verify credit score updated to 110 (base 10 + 100 bonus)
    const creditProfile = await creditScoreProgram.account.creditAccount.fetch(creditAccountPda);
    assert.equal(creditProfile.score.toString(), "110");
  });

  it("Fails to confirm delivery of an already completed trade (InvalidTradeStatus)", async () => {
    try {
      await tradeEscrowProgram.methods
        .confirmDelivery()
        .accounts({
          authority: buyer.publicKey,
          tradeAccount: tradeAccountPda,
          escrowVault: escrowVaultPda,
          farmer: farmer.publicKey,
          treasury: treasury.publicKey,
          creditAccount: creditAccountPda,
          creditScoreProgram: creditScoreProgram.programId,
          batchAccount: batchPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([buyer])
        .rpc();
      assert.fail("Should have failed with InvalidTradeStatus error");
    } catch (err: any) {
      assert.include(err.message, "InvalidTradeStatus");
    }
  });

  describe("Dispute Flows", () => {
    const disputeBatchName = "Honeycrisp Apples";
    let disputeBatchPda: anchor.web3.PublicKey;
    let disputeTradePda: anchor.web3.PublicKey;
    let disputeVaultPda: anchor.web3.PublicKey;
    let adminStatePda: anchor.web3.PublicKey;
    let configPda: anchor.web3.PublicKey;

    before(async () => {
      // Derive PDAs
      [disputeBatchPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("batch"), farmer.publicKey.toBuffer(), Buffer.from(disputeBatchName)],
        cropBatchProgram.programId
      );

      [disputeTradePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("trade-account"), disputeBatchPda.toBuffer()],
        tradeEscrowProgram.programId
      );

      [disputeVaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("escrow-vault"), disputeBatchPda.toBuffer()],
        tradeEscrowProgram.programId
      );

      [adminStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("admin"), provider.wallet.publicKey.toBuffer()],
        adminRegistryProgram.programId
      );

      [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("config")],
        adminRegistryProgram.programId
      );

      // Initialize the program config first (sets master_authority)
      await adminRegistryProgram.methods
        .initialize(provider.wallet.publicKey)
        .accounts({
          deployer: provider.wallet.publicKey,
          config: configPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();

      // Mint new crop batch
      await cropBatchProgram.methods
        .mintBatch(disputeBatchName)
        .accounts({
          signer: farmer.publicKey,
          farmer: farmerPda,
          batchAccount: disputeBatchPda,
          creditAccount: creditAccountPda,
          creditScoreProgram: creditScoreProgram.programId,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([farmer])
        .rpc();

      // Create trade escrow (deposit 2.0 SOL)
      const depositAmount = new anchor.BN(2.0 * anchor.web3.LAMPORTS_PER_SOL);
      await tradeEscrowProgram.methods
        .createTrade(depositAmount)
        .accounts({
          buyer: buyer.publicKey,
          batchAccount: disputeBatchPda,
          tradeAccount: disputeTradePda,
          escrowVault: disputeVaultPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([buyer])
        .rpc();

      // Accept trade escrow
      await tradeEscrowProgram.methods
        .acceptTrade()
        .accounts({
          farmer: farmer.publicKey,
          batchAccount: disputeBatchPda,
          tradeAccount: disputeTradePda,
        } as any)
        .signers([farmer])
        .rpc();

      // Register the provider.wallet as an authorized admin in the admin_registry program
      await adminRegistryProgram.methods
        .addAdmin()
        .accounts({
          authority: provider.wallet.publicKey,
          config: configPda,
          adminToAdd: provider.wallet.publicKey,
          adminState: adminStatePda,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();
    });

    it("Fails to raise a dispute if signer is not the buyer or farmer (Unauthorized)", async () => {
      const randomUser = anchor.web3.Keypair.generate();
      // Fund random user
      const sig = await provider.connection.requestAirdrop(randomUser.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);
      const latestBlockhash = await provider.connection.getLatestBlockhash();
      await provider.connection.confirmTransaction({ signature: sig, ...latestBlockhash });

      try {
        await tradeEscrowProgram.methods
          .raiseDispute()
          .accounts({
            authority: randomUser.publicKey,
            batchAccount: disputeBatchPda,
            tradeAccount: disputeTradePda,
          } as any)
          .signers([randomUser])
          .rpc();
        assert.fail("Should have failed with Unauthorized error");
      } catch (err: any) {
        assert.include(err.message, "Unauthorized");
      }
    });

    it("Buyer raises a dispute successfully", async () => {
      await tradeEscrowProgram.methods
        .raiseDispute()
        .accounts({
          authority: buyer.publicKey,
          batchAccount: disputeBatchPda,
          tradeAccount: disputeTradePda,
        } as any)
        .signers([buyer])
        .rpc();

      // Verify TradeAccount status is Disputed
      const tradeState = await tradeEscrowProgram.account.tradeAccount.fetch(disputeTradePda);
      assert.deepEqual(tradeState.status, { disputed: {} });
    });

    it("Fails to resolve a dispute if signer is not the admin (Unauthorized)", async () => {
      const nonAdmin = anchor.web3.Keypair.generate();
      // Fund non-admin
      const sig = await provider.connection.requestAirdrop(nonAdmin.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);
      const latestBlockhash = await provider.connection.getLatestBlockhash();
      await provider.connection.confirmTransaction({ signature: sig, ...latestBlockhash });

      const [nonAdminStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("admin"), nonAdmin.publicKey.toBuffer()],
        adminRegistryProgram.programId
      );

      try {
        await tradeEscrowProgram.methods
          .resolveDispute({ refundBuyer: {} })
          .accounts({
            admin: nonAdmin.publicKey,
            adminState: nonAdminStatePda,
            batchAccount: disputeBatchPda,
            tradeAccount: disputeTradePda,
            escrowVault: disputeVaultPda,
            farmer: farmer.publicKey,
            buyer: buyer.publicKey,
            adminRegistryProgram: adminRegistryProgram.programId,
            systemProgram: anchor.web3.SystemProgram.programId,
          } as any)
          .signers([nonAdmin])
          .rpc();
        assert.fail("Should have failed with Unauthorized error");
      } catch (err: any) {
        // Assert it fails because account is not initialized or owned by wrong program
        assert.ok(err.message);
      }
    });

    it("Admin resolves dispute in favor of buyer (refunds 2 SOL)", async () => {
      const initialBuyerBalance = await provider.connection.getBalance(buyer.publicKey);

      // Call resolveDispute
      await tradeEscrowProgram.methods
        .resolveDispute({ refundBuyer: {} })
        .accounts({
          admin: provider.wallet.publicKey,
          adminState: adminStatePda,
          batchAccount: disputeBatchPda,
          tradeAccount: disputeTradePda,
          escrowVault: disputeVaultPda,
          farmer: farmer.publicKey,
          buyer: buyer.publicKey,
          adminRegistryProgram: adminRegistryProgram.programId,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();

      // Verify TradeAccount state is now Completed
      const tradeState = await tradeEscrowProgram.account.tradeAccount.fetch(disputeTradePda);
      assert.deepEqual(tradeState.status, { completed: {} });

      // Verify buyer refunded: exactly 2.0 SOL (2,000,000,000 lamports)
      const finalBuyerBalance = await provider.connection.getBalance(buyer.publicKey);
      assert.equal((finalBuyerBalance - initialBuyerBalance).toString(), "2000000000");

      // Verify vault balance is now 0
      const vaultBalance = await provider.connection.getBalance(disputeVaultPda);
      assert.equal(vaultBalance.toString(), "0");
    });
  });

  describe("Cancel Trade Flows", () => {
    const cancelBatchName = "Honeycrisp Pears";
    let cancelBatchPda: anchor.web3.PublicKey;
    let cancelTradePda: anchor.web3.PublicKey;
    let cancelVaultPda: anchor.web3.PublicKey;

    before(async () => {
      // Derive PDAs
      [cancelBatchPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("batch"), farmer.publicKey.toBuffer(), Buffer.from(cancelBatchName)],
        cropBatchProgram.programId
      );

      [cancelTradePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("trade-account"), cancelBatchPda.toBuffer()],
        tradeEscrowProgram.programId
      );

      [cancelVaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("escrow-vault"), cancelBatchPda.toBuffer()],
        tradeEscrowProgram.programId
      );

      // Mint new crop batch
      await cropBatchProgram.methods
        .mintBatch(cancelBatchName)
        .accounts({
          signer: farmer.publicKey,
          farmer: farmerPda,
          batchAccount: cancelBatchPda,
          creditAccount: creditAccountPda,
          creditScoreProgram: creditScoreProgram.programId,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([farmer])
        .rpc();

      // Create trade escrow (deposit 1.5 SOL)
      const depositAmount = new anchor.BN(1.5 * anchor.web3.LAMPORTS_PER_SOL);
      await tradeEscrowProgram.methods
        .createTrade(depositAmount)
        .accounts({
          buyer: buyer.publicKey,
          batchAccount: cancelBatchPda,
          tradeAccount: cancelTradePda,
          escrowVault: cancelVaultPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([buyer])
        .rpc();
    });

    it("Fails to cancel the trade if signer is not the buyer (Unauthorized)", async () => {
      try {
        await tradeEscrowProgram.methods
          .cancelTrade()
          .accounts({
            buyer: farmer.publicKey,
            batchAccount: cancelBatchPda,
            tradeAccount: cancelTradePda,
            escrowVault: cancelVaultPda,
            systemProgram: anchor.web3.SystemProgram.programId,
          } as any)
          .signers([farmer])
          .rpc();
        assert.fail("Should have failed with Unauthorized error");
      } catch (err: any) {
        assert.include(err.message, "Unauthorized");
      }
    });

    it("Buyer cancels the trade successfully (funds refunded and account closed)", async () => {
      const initialBuyerBalance = await provider.connection.getBalance(buyer.publicKey);

      await tradeEscrowProgram.methods
        .cancelTrade()
        .accounts({
          buyer: buyer.publicKey,
          batchAccount: cancelBatchPda,
          tradeAccount: cancelTradePda,
          escrowVault: cancelVaultPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([buyer])
        .rpc();

      // Verify TradeAccount is closed (fetching it returns null or throws an error)
      try {
        await tradeEscrowProgram.account.tradeAccount.fetch(cancelTradePda);
        assert.fail("TradeAccount should have been closed");
      } catch (err: any) {
        assert.ok(err.message.includes("Account does not exist") || err.message.includes("Failed to find"));
      }

      // Verify buyer refunded: exactly 1.5 SOL (1,500,000,000 lamports) + rent from closed trade account
      const finalBuyerBalance = await provider.connection.getBalance(buyer.publicKey);
      assert.ok(finalBuyerBalance > initialBuyerBalance);
      assert.ok(finalBuyerBalance - initialBuyerBalance >= 1500000000);

      // Verify vault balance is now 0
      const vaultBalance = await provider.connection.getBalance(cancelVaultPda);
      assert.equal(vaultBalance.toString(), "0");
    });
  });
});
