import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaTwitter } from "../target/types/solana_twitter";

describe("solana-twitter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaTwitter as Program<SolanaTwitter>;

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("can send a tweet!", async () => {
    const tweet = anchor.web3.Keypair.generate();
    const anchorProvider = program.provider as anchor.AnchorProvider

    // Add your test here.
    await program.methods
  .sendTweet('veganism', 'Hummus, am I right?')
  .accounts({
    tweet: tweet.publicKey,
    author: anchorProvider.wallet.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .signers([tweet])
  .rpc()
    

  const tweetAccount = await program.account.tweet.fetch(tweet.publicKey);
  console.log(tweetAccount);
});
});
