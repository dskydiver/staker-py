// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.
const idl = require("../target/idl/staker.json");
const web3 = require("@solana/web3.js");
const anchor = require("@coral-xyz/anchor");
const token = require("@solana/spl-token");

const programId = "5KPMSJEraFFCrxHiB2mXVQQzNft6spohjEtwVfYzEqd3";

module.exports = async function (provider) {
  try {
    // Configure client to use the provider.
    anchor.setProvider(provider);
    const program = new anchor.Program(idl, programId, provider);

    const [pool_token_pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pool_token"), web3.SystemProgram.programId.toBuffer()],
      program.programId
    );
    const [vault_sol_pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault_sol"), web3.SystemProgram.programId.toBuffer()],
      program.programId
    );

    const [vault_pool_token_pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("vault_pool_token"),
        web3.SystemProgram.programId.toBuffer(),
      ],
      program.programId
    );

    const [pool_pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pool")],
      program.programId
    );

    console.log("initializing...");

    // create a new staker account for SOL
    await program.methods
      .poolInit()
      .accounts({
        pool: pool_pda,
        sender: provider.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    await program.methods
      .init()
      .accounts({
        poolToken: pool_token_pda,
        vaultSol: vault_sol_pda,
        vaultPoolToken: vault_pool_token_pda,
        payer: provider.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
        rent: web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();
  } catch (e) {
    console.log(e);
  }
};
