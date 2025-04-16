import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../Turbin3-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    let tx = await createNft(umi, {
        mint,
        name: "Crazyyy Jeff",
        uri: "https://devnet.irys.xyz/9TU7c8wjxMt6Q2Hm9xpcqKo5hGUd74Nn5tyNS5XcS6F9",
        sellerFeeBasisPoints: percentAmount(5),

    })
     let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();

// https://explorer.solana.com/tx/2boRtPvwBT2tHRbsT9499LkWuruW4ndxsKaeLvW75kkJHXoGLx2sqUw6ZkmiQNBBoFVyvyJzZZDZFDWCEm7qHCD3?cluster=devnet 
// Mint Address:  weMWRAEK26jrT5rieBgCz4sWJygU4juCvmwGTk7v6Bf 