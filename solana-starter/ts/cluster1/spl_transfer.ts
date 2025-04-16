import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../Turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("2YJUC77zvSeGV6whsTx3M8Q3Yiw5f8WPhaFQVUjoxhWR");

// Recipient address
const to = new PublicKey("9Eis2fKZpAcZyVSbDdnuBZJ2f74SkSjAC7dx1GsAhNwz");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        );

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        );

        // Transfer the new token to the "toTokenAccount" we just created
        const tx = await transfer(
            connection,
            keypair,
            fromTokenAccount.address,
            toTokenAccount.address,
            keypair.publicKey,
            1 
        );

        console.log(`Transfer successful: https://explorer.solana.com/tx/${tx}?cluster=devnet`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();

//https://explorer.solana.com/tx/2RNRatm2HHnFRLRfDFsCVuNE9mPgcghyHeTcqxpW7WheEHKygYAybRBAF7myRtTSXwNUhE3HdkwgKzfEdjxJzNhp?cluster=devnet