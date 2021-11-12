import assert from "assert";
import * as anchor from "@project-serum/anchor";

describe("puppet", () => {
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.Puppet;

  let programAccount: anchor.web3.Keypair;

  it("Performs CPI from master to puppet", async () => {
    const puppet = anchor.workspace.Puppet;
    const puppetMaster = anchor.workspace.PuppetMaster;

    // Initialize a new puppet account
    const newPuppetAccount = anchor.web3.Keypair.generate();
    const tx = await puppet.rpc.initialize({
      accounts: {
        puppet: newPuppetAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [newPuppetAccount],
    });
    console.log("Your transaction signature", tx);

    // asserts
    let puppetAccount = await puppet.account.announcement.fetch(
      newPuppetAccount.publicKey
    );
    assert.deepEqual(puppetAccount.text, Buffer.from(""));

    await puppet.rpc.setAnnouncement(Buffer.from("Hello Solana!"), {
      accounts: {
        announcement: newPuppetAccount.publicKey,
      },
    });

    // asserts
    puppetAccount = await puppet.account.announcement.fetch(
      newPuppetAccount.publicKey
    );
    assert.deepEqual(puppetAccount.text, Buffer.from("Hello Solana!"));

    await puppetMaster.rpc.pullString(Buffer.from("Hello world!"), {
      accounts: {
        puppet: newPuppetAccount.publicKey,
        puppetProgram: puppet.programId,
      },
    });

    // asserts
    puppetAccount = await puppet.account.announcement.fetch(
      newPuppetAccount.publicKey
    );
    assert.deepEqual(puppetAccount.text, Buffer.from("Hello world!"));
  });
});
