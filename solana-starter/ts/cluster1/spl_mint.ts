import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from "../Turbin3-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;

// Mint address
const mint = new PublicKey("2YJUC77zvSeGV6whsTx3M8Q3Yiw5f8WPhaFQVUjoxhWR");

(async () => {
    try {
        const ata = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey,
        )

        console.log(`ATA: ${ata.address}`);

        //Mint to ATA:

        const mintTx = await mintTo(
            connection,
            keypair,
            mint,
            ata.address,
            keypair.publicKey,
            token_decimals * 1000n,
        )

        console.log(`Mint transaction: ${mintTx}`);
        
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()

//ATA: 7TvPdiyuivkV64jbsd1qGGEESmynSWRzfW6Y7JRyP4Xq
// Mint transaction: 4ddoD6MdfwZruUUWaWCuPxn6HXbojEGzuAKLSWzwPoU84bbEjAEMpJxErbwjpDfYYmnyGdjxN7oGAfwdjN95N5fJ