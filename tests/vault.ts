import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { Vault } from "../target/types/vault"; // <-- Match your program name in Cargo.toml

describe("vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Vault as Program<Vault>;
  const user = provider.wallet;

  let vaultStatePda: PublicKey;
  let vaultPda: PublicKey;

  before(async () => {
    [vaultStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault_state"), user.publicKey.toBuffer()],
      program.programId
    );

    [vaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), vaultStatePda.toBuffer()],
      program.programId
    );
  });

  it("initializes the vault", async () => {
    const tx = await program.methods
      .initializeVault()
      .accountsPartial({
        user: user.publicKey,
        vaultState: vaultStatePda,
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Vault initialized with tx:", tx);
  });

  it("deposits 1 SOL into the vault", async () => {
    const amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);
    const tx = await program.methods
      .deposit(amount)
      .accountsPartial({
        user: user.publicKey,
        vaultState: vaultStatePda,
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Deposit successful with tx:", tx);
  });

  it("withdraws 0.5 SOL from the vault", async () => {
    const amount = new anchor.BN(0.5 * anchor.web3.LAMPORTS_PER_SOL);
    const tx = await program.methods
      .withdraw(amount)
      .accountsPartial({
        user: user.publicKey,
        vaultState: vaultStatePda,
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Withdraw successful with tx:", tx);
  });

  it("closes the vault", async () => {
    const tx = await program.methods
      .close()
      .accountsPartial({
        user: user.publicKey,
        vaultState: vaultStatePda,
        vault: vaultPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Vault closed with tx:", tx);
  });
});
