import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MemoAnchor } from "../target/types/memo_anchor";

describe("memo_anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.MemoAnchor as Program<MemoAnchor>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
