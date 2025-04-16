import wallet from "../Turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image =  "https://devnet.irys.xyz/9DRa9dwvBQook4wuSuDLCa6HPMTTFShZvAfEFeSBYMbi"
        const metadata = {
            name: "Crazyy Jeff",
            symbol: "CJ",
            description: "Jeff with cool shades",
            image: image,
            attributes: [
                {trait_type: 'Shades', value: 'black'},
                {trait_type: 'Hat', value: 'black'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "?"
                    },
                ]
            },
            creators: []
        };

        const file = createGenericFile(JSON.stringify(metadata), "metadata.json", {contentType: "application/json"});
        const myUri = await umi.uploader.upload([file]);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();

// https://devnet.irys.xyz/9TU7c8wjxMt6Q2Hm9xpcqKo5hGUd74Nn5tyNS5XcS6F9