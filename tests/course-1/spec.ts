import * as anchor from "@project-serum/anchor";
import { Course1 } from "../target/types/course_1';";

describe("course 1", () => {
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Course1 as Program<Course1>;

  it("Initialize", async () => {
    // Simple transaction
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
