import * as anchor from "@project-serum/anchor";
import {
  AnchorProvider,
  BN,
  Program,
  setProvider,
  Spl,
  utils,
  web3,
} from "@project-serum/anchor";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";
import { expect } from "chai";
import { TheShareProg } from "../target/types/the_share_prog";
import { initializeAccount, initializeMint, mintTo } from "./pretest";

describe("the-share-prog", () => {
  // Configure the client to use the local cluster.
  const provider = AnchorProvider.env();
  setProvider(provider);
  const token = new web3.Keypair();
  let tokenAccount: web3.PublicKey;
  const spl = Spl.token();

  const plan = new web3.Keypair();
  const request = new web3.Keypair();

  let treasurer: web3.PublicKey;
  let treasury: web3.PublicKey;

  const program = anchor.workspace.TheShareProg as Program<TheShareProg>;

  before(async () => {
    // Init mints
    await initializeMint(6, token, spl);
    // Init accounts
    tokenAccount = await anchor.utils.token.associatedAddress({
      mint: token.publicKey,
      owner: provider.wallet.publicKey,
    });
    await initializeAccount(
      tokenAccount,
      token.publicKey,
      provider.wallet.publicKey,
      provider
    );

    // Mint tokens
    await mintTo(new BN("1000000000000"), token.publicKey, tokenAccount, spl);

    // Derive treasury & treasurer
    const [treasurerPublicKey] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("treasurer"), plan.publicKey.toBuffer()],
      program.programId
    );
    treasurer = treasurerPublicKey;
    treasury = await utils.token.associatedAddress({
      mint: token.publicKey,
      owner: treasurer,
    });
  });

  it("Is plan created!", async () => {
    // Add your test here.
    console.log("pusshshsh=> ", provider.wallet.publicKey);
    const tx = await program.methods
      .createPlan(new BN("300000000000"), "The first plan", [
        new web3.PublicKey("7APrphoctygdeDZsrNH4r1LMXwuM9WsiYiq8BXXcpQ1Y"),
        new web3.PublicKey("H6JfJrmWXcJZjKQQYBPThZmo9ZBkdm3jLcMQnrwwMDQw"),
      ])
      .accounts({
        planer: provider.wallet.publicKey,
        plan: plan.publicKey,
        token: token.publicKey,
        treasurer,
        ataPlaner: tokenAccount,
        treasury,
      })
      .signers([plan])
      .rpc();
    console.log("Your transaction signature", tx);
    expect(tx).to.be.an("string");
  });

  it("Get data #1", async () => {
    const { fund, planName } = await program.account.plan.fetch(plan.publicKey);
    console.log("thong tin plan =>", fund, planName);
    expect(fund.eq(new BN("300000000000"))).to.be.true;
  });

  it("Is plan changed!", async () => {
    // Add your test here.
    const tx = await program.methods
      .changePlanConfigs(new BN("400000000000"), "The  plan changed", [
        new web3.PublicKey("7APrphoctygdeDZsrNH4r1LMXwuM9WsiYiq8BXXcpQ1Y"),
        new web3.PublicKey("2Mdbxjidw1oHPwkSsFqfaFcYHjLUrNdkVt98Xc1K5dac"),
        new web3.PublicKey("H6JfJrmWXcJZjKQQYBPThZmo9ZBkdm3jLcMQnrwwMDQw"),
      ])
      .accounts({
        planer: provider.wallet.publicKey,
        plan: plan.publicKey,
        ataPlaner: tokenAccount,
        treasury,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    expect(tx).to.be.an("string");
  });

  it("Get data #2", async () => {
    const { fund, planName } = await program.account.plan.fetch(plan.publicKey);
    console.log(" plan changed =>", fund, planName);
    expect(fund.eq(new BN("400000000000"))).to.be.true;
  });

  it("Is request created!", async () => {
    // Add your test here.
    const tx = await program.methods
      .createRequest(new BN("40000000000"), "The  first request")
      .accounts({
        withdrawer: provider.wallet.publicKey,
        plan: plan.publicKey,
        request: request.publicKey,
      })
      .signers([request])
      .rpc();
    console.log("Your transaction signature", tx);
    expect(tx).to.be.an("string");
  });

  it("Get data #3", async () => {
    const { amount, reason } = await program.account.request.fetch(
      request.publicKey
    );
    console.log(" request created =>", amount, reason);
    expect(amount.eq(new BN("40000000000"))).to.be.true;
  });

  it("Is request changed!", async () => {
    // Add your test here.
    const tx = await program.methods
      .changeRequest(new BN("20000000000"), "The  changed requested")
      .accounts({
        withdrawer: provider.wallet.publicKey,
        plan: plan.publicKey,
        request: request.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    expect(tx).to.be.an("string");
  });

  it("Get data #4", async () => {
    const { amount, reason } = await program.account.request.fetch(
      request.publicKey
    );
    console.log(" request created =>", amount, reason);
    expect(amount.eq(new BN("20000000000"))).to.be.true;
  });

  it("Is request accepted!", async () => {
    // Add your test here.
    const tx = await program.methods
      .acceptRequest()
      .accounts({
        planer: provider.wallet.publicKey,
        plan: plan.publicKey,
        request: request.publicKey,
        ataPlaner: tokenAccount,
        ataRequester: tokenAccount,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    expect(tx).to.be.an("string");
  });

  it("Get data #5", async () => {
    const { state } = await program.account.request.fetch(request.publicKey);
    const { fund } = await program.account.plan.fetch(plan.publicKey);
    console.log(" request confirmed =>", state);
    expect(fund.eq(new BN("400000000000").sub(new BN("20000000000")))).to.be
      .true;
  });

  it("Is request rejected!", async () => {
    // Add your test here.
    const tx = await program.methods
      .rejectRequest()
      .accounts({
        withdrawer: provider.wallet.publicKey,
        plan: plan.publicKey,
        request: request.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    expect(tx).to.be.an("string");
  });

  it("Get data #6", async () => {
    const { state } = await program.account.request.fetch(request.publicKey);
    console.log(" request confirmed =>", state);
  });

  it("Is request canceled!", async () => {
    // Add your test here.
    const tx = await program.methods
      .cancelRequest()
      .accounts({
        withdrawer: provider.wallet.publicKey,
        plan: plan.publicKey,
        request: request.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    expect(tx).to.be.an("string");
  });

  it("Get data #7", async () => {
    const { state } = await program.account.request.fetch(request.publicKey);
    console.log(" request confirmed =>", state);
  });
});
