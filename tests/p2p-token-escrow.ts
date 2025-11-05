import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { P2pTokenEscrow } from "../target/types/p2p_token_escrow";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import { ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAccount, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token"
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { should } from "chai";

describe("p2p-token-escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.p2pTokenEscrow as Program<P2pTokenEscrow>;
  const conn = program.provider.connection;
  const maker = Keypair.generate();
  const taker = Keypair.generate();
  let mintMaker: PublicKey;
  let mintTaker: PublicKey;

  const maker_offer: number = 10;
  const seed: number = 0;
  const maker_ask: number = 20;

  // try {
  //   const devkeyPath = homedir() + "/.config/solana/devkey.json";
  //   const outputPath = homedir() + "/.config/solana/devkey-bytes.json";
  //   const pk = readFileSync(devkeyPath, 'utf-8').trim();
// 
//     const bs58 = require('bs58');
//     const fs = require('fs');
//     const b = bs58.decode(pk);
//     const j = new Uint8Array(b.buffer, b.byteOffset, b.byteLength / Uint8Array.BYTES_PER_ELEMENT);
//     fs.writeFileSync(outputPath, `[${j}]`);
//   }
//   catch (e) {
//     should().fail(e);
//   }


  it("Init mints", async () => {
    await airdrop(maker.publicKey, LAMPORTS_PER_SOL);
    await airdrop(taker.publicKey, LAMPORTS_PER_SOL);

    mintMaker = await createMint(conn, maker, maker.publicKey, null, 6);
    mintTaker = await createMint(conn, taker, taker.publicKey, null, 6);

    await mintTo(
      conn,
      maker,
      mintMaker,
      (await getOrCreateAssociatedTokenAccount(conn, maker, mintMaker, maker.publicKey, true)).address,
      maker,
      1000000000
    );
    await mintTo(
      conn,
      taker,
      mintTaker,
      (await getOrCreateAssociatedTokenAccount(conn, taker, mintTaker, taker.publicKey, true)).address,
      taker,
      1000000000
    );

  });

  it("Create make", async () => {

    const escrowPDA = getEscrowPDA(maker.publicKey, seed);
    const escrowVaultPDA = getAssociatedTokenAddressSync(mintMaker, escrowPDA, true);

    await program.methods
      .make(new anchor.BN(seed), new anchor.BN(maker_offer), new anchor.BN(maker_ask))
      .accountsPartial({
        maker: maker.publicKey,
        makerAta: getAssociatedTokenAddressSync(mintMaker, maker.publicKey),
        escrow: escrowPDA,
        escrowVault: escrowVaultPDA,
        mintMaker, 
        mintTaker,
        systemProgram: SYSTEM_PROGRAM_ID,
        mintMakerTokenProgram: TOKEN_PROGRAM_ID,
        mintTakerTokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc({commitment: "confirmed"})

    const escrowVault = await getOrCreateAssociatedTokenAccount(conn, maker, mintMaker, escrowPDA, true);
    should().equal(escrowVault.amount, BigInt(maker_offer), "Tokens did not transfer to vault");
  });


  it("Refund make", async () => {
    const seed_refund = 67;
    const escrowPDA = getEscrowPDA(maker.publicKey, seed_refund);
    const escrowVaultPDA = getAssociatedTokenAddressSync(mintMaker, escrowPDA, true);

    await program.methods
      .make(new anchor.BN(seed_refund), new anchor.BN(maker_offer), new anchor.BN(maker_ask))
      .accountsPartial({
        maker: maker.publicKey,
        makerAta: getAssociatedTokenAddressSync(mintMaker, maker.publicKey),
        escrow: escrowPDA,
        escrowVault: escrowVaultPDA,
        mintMaker, 
        mintTaker,
        systemProgram: SYSTEM_PROGRAM_ID,
        mintMakerTokenProgram: TOKEN_PROGRAM_ID,
        mintTakerTokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc({commitment: "confirmed"})

    await program.methods
      .refund(new anchor.BN(seed_refund))
      .accountsPartial({
        maker: maker.publicKey,
        escrow: escrowPDA,
        escrowVault: escrowVaultPDA,
        mint: mintMaker,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc({commitment: "confirmed"});

    try {
      should().equal(await conn.getAccountInfo(escrowPDA), null, "Failed to delete escrow account");
    }
    catch (e) {}

    try {
      should().equal(await conn.getAccountInfo(escrowVaultPDA), null, "Failed to delete escrow vault account");
    }
    catch (e) {}
  });

  it("Take make", async () => {

    const escrowPDA = getEscrowPDA(maker.publicKey, seed);
    const escrowVaultPDA = getAssociatedTokenAddressSync(mintMaker, escrowPDA, true);
     
    await program.methods
    .take(new anchor.BN(seed), new anchor.BN(maker_offer), new anchor.BN(maker_ask))
    .accountsPartial({
      taker: taker.publicKey,
      takerAtaFrom: getAssociatedTokenAddressSync(mintTaker, taker.publicKey),
      takerAtaTo: getAssociatedTokenAddressSync(mintMaker, taker.publicKey),
      maker: maker.publicKey,
      makerAtaTo: getAssociatedTokenAddressSync(mintTaker, maker.publicKey),
      escrow: escrowPDA,
      escrowVault: escrowVaultPDA,
      mintMaker,
      mintTaker,
      mintMakerTokenProgram: TOKEN_PROGRAM_ID,
      mintTakerTokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([taker])
    .rpc({commitment: "confirmed"});

    // Validation
    const makerAtaTo = await getOrCreateAssociatedTokenAccount(conn, maker, mintTaker, maker.publicKey);
    const takerAtaTo = await getOrCreateAssociatedTokenAccount(conn, taker, mintMaker, taker.publicKey);

    should().equal(makerAtaTo.amount, BigInt(maker_ask), "Maker did not get their tokens");
    should().equal(takerAtaTo.amount, BigInt(maker_offer), "Taker did not get their tokens");
  });

  async function airdrop(to: PublicKey, lamports: number) {
    const tx = await conn.requestAirdrop(to, lamports);
    const blockhash = await conn.getLatestBlockhash();
    await conn.confirmTransaction(
      {
        signature: tx, 
        blockhash: blockhash.blockhash,
        lastValidBlockHeight: blockhash.lastValidBlockHeight
      }, 
      "confirmed");
  }

  function getEscrowPDA(maker: PublicKey, seed: number): PublicKey {
    return PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("escrow"),
        maker.toBuffer(),
        new anchor.BN(seed).toArrayLike(Buffer, "le", 8), 
      ],
    program.programId)[0];
  }
});
