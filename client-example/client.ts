import fs from "fs";
import { homedir } from "os";
import { clusterApiUrl, Connection, Keypair, PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import type { P2pTokenEscrow } from "../target/types/p2p_token_escrow";
import idl from "../target/idl/p2p_token_escrow.json";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { off } from "process";

const main = async () => {
  const wallet = new anchor.Wallet(getKeypairFromFile(homedir() + "/.config/solana/dev.json"));
  const conn = new Connection(clusterApiUrl("devnet"), "confirmed");
  const provider = new anchor.AnchorProvider(conn, wallet, {commitment: "confirmed"});
  anchor.setProvider(provider);
  const program = new anchor.Program<P2pTokenEscrow>(idl as P2pTokenEscrow, provider);
  
  await createMake(program);

  await takeMake(program);
};

async function createMake(program: anchor.Program<P2pTokenEscrow>) {
  const maker = getKeypairFromFile(homedir() + "/.config/solana/maker.json");

  const seed = new anchor.BN(0);

  const offerAmount = new anchor.BN(1); // How much the maker offers
  const offerMint = getKeypairFromFile(homedir() + "/.config/solana/makerMint.json"); // of which mint

  const askAmount = new anchor.BN(1);   // How much the maker asks for
  const askMint = getKeypairFromFile(homedir() + "/.config/solana/takerMint.json");   // of which mint

  const tx = await program
      .methods
      .make(seed, offerAmount, askAmount)
      .accounts({
      maker: maker.publicKey,
      mintMaker: offerMint.publicKey,
      mintTaker: askMint.publicKey,
      mintMakerTokenProgram: TOKEN_PROGRAM_ID,
      mintTakerTokenProgram: TOKEN_PROGRAM_ID,
    })
      .signers([maker])
      .rpc({commitment: "confirmed"});  
  console.log(tx);
}

async function takeMake(program: anchor.Program<P2pTokenEscrow>) {
  const maker = getKeypairFromFile(homedir() + "/.config/solana/maker.json");
  const taker = getKeypairFromFile(homedir() + "/.config/solana/taker.json");

  const seed = new anchor.BN(0);

  const offerAmount = new anchor.BN(1); // How much the maker offers
  const offerMint = getKeypairFromFile(homedir() + "/.config/solana/makerMint.json"); // of which mint

  const askAmount = new anchor.BN(1);   // How much the maker asks for
  const askMint = getKeypairFromFile(homedir() + "/.config/solana/takerMint.json");   // of which mint

  const tx = await program.methods
    .take(seed, offerAmount, askAmount)
    .accountsPartial({
      taker: taker.publicKey,
      maker: maker.publicKey,
      mintMaker: offerMint.publicKey,
      mintTaker: askMint.publicKey,
      mintMakerTokenProgram: TOKEN_PROGRAM_ID,
      mintTakerTokenProgram: TOKEN_PROGRAM_ID,
    })
    .rpc({commitment: "confirmed"});

  console.log(tx);
}

async function refund(program: anchor.Program<P2pTokenEscrow>){
  const maker = getKeypairFromFile(homedir() + "/.config/solana/maker.json");
  const offerMint = getKeypairFromFile(homedir() + "/.config/solana/makerMint.json"); // of which mint
  
  const seed = new anchor.BN(0);
  const tx = await program.methods
  .refund(seed)
  .accounts({
    maker: maker.publicKey, 
    mint: offerMint.publicKey,
    tokenProgram: TOKEN_PROGRAM_ID
  })
  .signers([maker])
  .rpc({commitment: "confirmed"});

  console.log(tx);
}

main().catch(console.error);

function getKeypairFromFile(path: string) {
  return Keypair.fromSecretKey(
    Uint8Array.from(
      JSON.parse(
        fs.readFileSync(path, "utf8").toString()
      )
    )
  );
}
