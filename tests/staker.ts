import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Staker } from "../target/types/staker";
import * as web3 from "@solana/web3.js";
import * as token from "@solana/spl-token";
import { NATIVE_MINT } from "@solana/spl-token";
describe("staker", async () => {
  try {
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Staker as Program<Staker>;
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
    let balance = await connection.getBalance(payer.publicKey);
    console.log("BOB sol amount", balance);

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

    // create a new staker account for SOL
    await program.methods
      .poolInit()
      .accounts({
        pool: pool_pda,
        sender: provider.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    // new wallet that store the token from pool
    let wallet_pool_token = await token.createAssociatedTokenAccount(
      connection,
      payer,
      pool_token_pda,
      provider.wallet.publicKey,
      null,
      token.TOKEN_PROGRAM_ID,
      token.ASSOCIATED_TOKEN_PROGRAM_ID
    );

    async function print_state() {
      let sol = await connection.getBalance(payer.publicKey);
      console.log("BOB SOL amount", sol);

      let balance = await connection.getTokenAccountBalance(wallet_pool_token);
      console.log("BOB LST amount", balance.value.amount);

      sol = await connection.getBalance(vault_sol_pda);
      console.log("VAULT sol amount", sol);

      balance = await connection.getTokenAccountBalance(vault_pool_token_pda);
      console.log("VAULT LST amount", balance.value.amount);
    }

    let operation_accounts = {
      poolTreasure: NATIVE_MINT,
      poolToken: pool_token_pda,
      poolTokenVault: vault_pool_token_pda,
      vaultSol: vault_sol_pda,
      sender: provider.wallet.publicKey,
      senderPoolToken: wallet_pool_token,
      tokenProgram: token.TOKEN_PROGRAM_ID,
      systemProgram: web3.SystemProgram.programId,
      pool: pool_pda,
    };
    await print_state();

    console.log("staking...");
    await program.methods
      .stake(new anchor.BN(1000)) // stake 0.001 sol
      .accounts(operation_accounts)
      .rpc();
    await print_state();

    console.log("unstaking...");
    await program.methods
      .unstake(new anchor.BN(10000)) // unstake 0.01 lst
      .accounts(operation_accounts)
      .rpc();

    await print_state();
  } catch (e) {
    console.log(e);
  }
});
