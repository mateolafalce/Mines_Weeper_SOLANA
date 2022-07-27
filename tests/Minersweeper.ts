import * as anchor from "@project-serum/anchor";
const { SystemProgram } = anchor.web3;

describe("Minersweeper", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Minersweeper;
  const rowColums = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    const tx = await program.rpc.create(
      provider.wallet.publicKey,
    //--------------------------------------------------------------------------------
    // 9 = ⬜​ unknown
    // 6 = 👁️‍🗨️ discovered and without bomb
    // 1 = 1️⃣ ​there is a mine 1 block around
    // 2 = 2️⃣ there is a mine 1 block around
    // 3 = 3️⃣ ​there is a mine 1 block around
    //--------------------------------------------------------------------------------
    // 8 = 🚩 red flag, here is a bomb
    // 7 = 👇 you select to discover what is in the box. the equivalent of a click
    //--------------------------------------------------------------------------------
    Buffer.from([9,9,9,9,9,9,9,9]),
    Buffer.from([9,9,9,9,9,9,9,9]),
    Buffer.from([9,9,9,9,9,9,9,9]),
    Buffer.from([9,9,9,9,9,9,9,9]),
    Buffer.from([9,9,9,9,9,9,9,9]),
    Buffer.from([9,9,9,9,9,9,9,9]),
    Buffer.from([9,9,9,9,9,9,9,9]),
    Buffer.from([9,9,9,9,9,9,9,9]),
    {
      accounts: {
        rowColums: rowColums.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [rowColums],
    });

    console.log("Your signature", tx);
  })
  
  

});
