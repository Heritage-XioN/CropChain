// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import * as fs from "fs";
import * as path from "path";

module.exports = async function (provider: anchor.AnchorProvider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  console.log("Starting deployment setup...");

  // Load MASTER_AUTHORITY from .env file (fallback to provider wallet if not found)
  let masterAuthorityKey = provider.wallet.publicKey.toBase58();
  try {
    const envPath = path.resolve(".env");
    if (fs.existsSync(envPath)) {
      const envContent = fs.readFileSync(envPath, "utf-8");
      const match = envContent.match(/MASTER_AUTHORITY\s*=\s*["']?([^"'\s]+)["']?/);
      if (match && match[1]) {
        masterAuthorityKey = match[1];
      }
    }
  } catch (err) {
    console.log("Could not read .env file, falling back to provider wallet.");
  }

  // 1. Initialize admin_registry ProgramConfig
  const idlPath = path.resolve("target/idl/admin_registry.json");
  if (!fs.existsSync(idlPath)) {
    console.log("Error: admin_registry IDL not found. Please build the program first.");
    return;
  }
  
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf-8"));
  const program = new Program(idl, provider);

  const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  const configInfo = await provider.connection.getAccountInfo(configPda);
  if (configInfo === null) {
    console.log(`Initializing admin_registry ProgramConfig...`);
    console.log(`Master Authority Address: ${masterAuthorityKey}`);
    
    await program.methods
      .initialize(new anchor.web3.PublicKey(masterAuthorityKey))
      .accounts({
        deployer: provider.wallet.publicKey,
        config: configPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();
      
    console.log("admin_registry Initialization successful!");
  } else {
    console.log("admin_registry ProgramConfig is already initialized on this cluster.");
  }

  // 2. Initialize credit_score CreditConfig
  const creditIdlPath = path.resolve("target/idl/credit_score.json");
  if (fs.existsSync(creditIdlPath)) {
    const creditIdl = JSON.parse(fs.readFileSync(creditIdlPath, "utf-8"));
    const creditProgram = new Program(creditIdl, provider);

    const [creditConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      creditProgram.programId
    );

    const creditConfigInfo = await provider.connection.getAccountInfo(creditConfigPda);
    if (creditConfigInfo === null) {
      console.log(`Initializing credit_score ProgramConfig...`);
      // Deployer is the authority, and trade_escrow program is trusted
      const tradeEscrowId = new anchor.web3.PublicKey("76vg8FiFH3hoT98ntU3Sb5apdZC3fQbr5mzzZLCgw1aF");
      await creditProgram.methods
        .initializeConfig(provider.wallet.publicKey, tradeEscrowId)
        .accounts({
          deployer: provider.wallet.publicKey,
          config: creditConfigPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();
      console.log("credit_score Initialization successful!");
    } else {
      console.log("credit_score ProgramConfig is already initialized on this cluster.");
    }
  }
}
