// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.
const idl = require("../target/idl/staker.json");
const web3 = require("@solana/web3.js");
const keypair = require("~/.config/solana/id.json");
const { bs58 } = require("@coral-xyz/anchor/dist/cjs/utils/bytes");
const anchor = require("@coral-xyz/anchor");
const token = require("@solana/spl-token");

const programId = "2xVHedjn1q57KTESiWTMA9GR4M7Pd5S21ssuurqfaFtd";

module.exports = async function (provider) {
  try {
    // Configure client to use the provider.
    anchor.setProvider(provider);
    const program = new anchor.Program(idl, programId, provider);
    console.log(program.programId.toString(), bs58.encode(keypair));

    let connection = provider.connection;
    let payer = web3.Keypair.generate();
    let airdrop = await connection.requestAirdrop(
      payer.publicKey,
      web3.LAMPORTS_PER_SOL * 100
    );
    const latestBlockHash = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdrop,
    });

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
