import * as anchor from "@project-serum/anchor";
import { Course1 } from "../target/types/course_2';";

describe("course 2", () => {
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Course2;

  it("Initialize", async () => {
    // Simple transaction
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
