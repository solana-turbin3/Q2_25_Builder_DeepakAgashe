import wallet from "../Turbin3-wallet.json"
import { createUmi} from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { PublicKey } from "@solana/web3.js";
import { buffer } from "stream/consumers";

import { TOKEN_PROGRAM_ID } from "@solana/spl-token";



// Define our Mint address
const mint = new PublicKey("2YJUC77zvSeGV6whsTx3M8Q3Yiw5f8WPhaFQVUjoxhWR");

const mint_umi = publicKey("2YJUC77zvSeGV6whsTx3M8Q3Yiw5f8WPhaFQVUjoxhWR");

const pda = PublicKey.findProgramAddressSync([Buffer.from("metadata"), mint.toBuffer()], TOKEN_PROGRAM_ID);

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
    try {
        // Start here
        
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            mint: mint_umi,
            mintAuthority: signer,
        }

        let data:   DataV2Args = {
            name: "Deepak",
            symbol: "D",
            uri: "",
            sellerFeeBasisPoints: 500,
            creators: null,
            collection: null,
            uses: null,
        }
        

        let args:   CreateMetadataAccountV3InstructionArgs = {
            data,
            isMutable: false,
            collectionDetails: null,

        }

        let tx = createMetadataAccountV3(umi, {
            ...accounts,
            ...args,
        })

        let result = await tx.sendAndConfirm(umi);
        console.log(bs58.encode(result.signature));
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();


//4gYZ3jr5SSrg5ZZEWvgtHYsA5c34y3DwiYBPhqHBd3GfFfQ4k6Ux31ehP2fYWMFQQUgg6AvHN42XHh7GUFMLsN3f