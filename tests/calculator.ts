import * as anchor from "@coral-xyz/anchor";
import { Program , BN} from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";
import {
  Keypair,
  SystemProgram,
  PublicKey,
  SYSVAR_RENT_PUBKEY
} from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Calculator as Program<Calculator>;

  // const Keypairpath = '../../keypair.json'; 
  // const buffer = require(Keypairpath);
  // const signerkeypair = Keypair.fromSecretKey(Uint8Array.from(buffer));
  const provider = anchor.AnchorProvider.env();
  const signer = provider.publicKey;
console.log(signer)
  it("sending inputs", async () => {

    const [vaultPDA, _] = await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('calc'),
        signer.toBuffer(),
      ],
      program.programId
    )

    const METADATA_SEED = "metadata";
    const TOKEN_METADATA_PROGRAM_ID =new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    const MINT_SEED = "mint";

    const metadata = {
      name: "Just a Test Token",
      symbol: "TEST",
      uri: "https://5vfxc4tr6xoy23qefqbj4qx2adzkzapneebanhcalf7myvn5gzja.arweave.net/7UtxcnH13Y1uBCwCnkL6APKsge0hAgacQFl-zFW9NlI",
      decimals: 9,
    };
    const mintAmount = 10;


    const [mint] = PublicKey.findProgramAddressSync(
      [Buffer.from(MINT_SEED)],
      program.programId
    );
    const [metadataAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from(METADATA_SEED),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    const destination = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: signer,
    });

    // try{
    //   const tx1 = await program.methods.initialize()
    // .accounts(
    //   {
    //     pdaAccount : vaultPDA,
    //     signer : signer,
    //   }
    // )
    // .rpc();

    // console.log("Your transaction signature", tx1);
    // }
    // catch(error){
    //   console.log(error);
    // }

    


    
  
  // try{
  //     const tx1 = await program.methods.initMint(metadata)
  //     .accounts({
  //       metadata: metadataAddress,
  //       mint : mint,
  //       payer : signer,
  //       rent : SYSVAR_RENT_PUBKEY,
  //       tokenProgram : anchor.utils.token.TOKEN_PROGRAM_ID,
  //       tokenMetadataProgram : TOKEN_METADATA_PROGRAM_ID
  //     })
  //   .rpc();

  //   console.log("Your transaction signature", tx1);
  //   }
  //   catch(error){
  //     console.log(error);
  //   }
  
  try{
    const tx1 = await program.methods.calculate({mul:{}}, 4, 3)
    .accounts({
      user : vaultPDA,
      mint : mint,
      destination : destination,
      signer : signer,
      rent : SYSVAR_RENT_PUBKEY,
      tokenProgram : anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram : anchor.utils.token.ASSOCIATED_PROGRAM_ID
    })
  .rpc();

  console.log("Your transaction signature", tx1);

  let data = await program.account.answer.fetch(vaultPDA);
  console.log("answer of calculation {}",data.result)
  }
  catch(error){
    console.log(error);
  }

  });


  
  
  
});
