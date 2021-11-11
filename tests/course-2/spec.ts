import assert from "assert";
import * as anchor from "@project-serum/anchor";
import { Course1 } from "../target/types/course_2';";

describe("course 2", () => {
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.Course2;

  let _programAccount: anchor.web3.Keypair;

  it("Initialize", async () => {
    const programAccount = anchor.web3.Keypair.generate();

    const tx = await program.rpc.initialize(Buffer.from("Hello world!"), {
      accounts: {
        announcement: programAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [programAccount],
    });
    console.log("Your transaction signature", tx);

    // Fetch the newly created account from the cluster.
    const account = await program.account.announcement.fetch(
      programAccount.publicKey
    );

    assert.deepEqual(account.text, Buffer.from("Hello world!"));

    // Store the account for the next test.
    _programAccount = programAccount;
  });

  it("Update", async () => {
    const programAccount = _programAccount;
    await program.rpc.update(Buffer.from("Hello Solana!"), {
      accounts: {
        announcement: programAccount.publicKey,
      },
    });

    // Fetch the newly created account from the cluster.
    const account = await program.account.announcement.fetch(
      programAccount.publicKey
    );
    assert.deepEqual(account.text, Buffer.from("Hello Solana!"));
  });
});
