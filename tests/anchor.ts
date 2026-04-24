import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import type { HardwareLedger } from "../target/types/hardware_ledger";

describe("Hardware Ledger Tests", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.HardwareLedger as anchor.Program<HardwareLedger>;
  
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // QA HOTFIX: Forzamos a TypeScript a ignorar sus tipos corruptos con 'any'
  // Esto elimina las líneas rojas y nos permite usar la sintaxis correcta.
  const program = anchor.workspace.HardwareLedger as any;
  const wallet = provider.wallet as anchor.Wallet;

  const timestamp = Date.now().toString().slice(-6);
  const modeloPieza = `RX6800-Test-${timestamp}`;
  const tipoPieza = "GPU";

  let workstationPda: anchor.web3.PublicKey;
  let componentePda: anchor.web3.PublicKey;

  it("Verifica o Crea la WorkstationDB", async () => {
    [workstationPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("workstation"), wallet.publicKey.toBuffer()],
      program.programId
    );

    try {
      await program.account.workstationDb.fetch(workstationPda, "confirmed");
      console.log("  -> Tu Workstation ya existe en la blockchain. Saltando inicialización.");
    } catch (e) {
      if (e.message && e.message.includes("Account does not exist")) {
        await program.methods
          .crearWorkstationDb("Master-Forge")
          .accounts({
            workstationDb: workstationPda,
            usuario: wallet.publicKey,
          })
          .rpc();
      } else {
        throw e;
      }
    }
  });

  it("Agrega un componente a la Workstation", async () => {
    [componentePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("componente"), wallet.publicKey.toBuffer(), Buffer.from(modeloPieza)],
      program.programId
    );

    await program.methods
      .agregarComponente(modeloPieza, tipoPieza, 5300, 1)
      .accounts({
        workstationDb: workstationPda,
        componente: componentePda,
        usuario: wallet.publicKey,
      })
      .rpc();

    const wsData = await program.account.workstationDb.fetch(workstationPda, "confirmed");
    const piezaData = await program.account.componente.fetch(componentePda, "confirmed");

    if (piezaData.modelo !== modeloPieza) throw new Error("Error: El modelo no se guardó correctamente.");
    if (wsData.componentes.length === 0) throw new Error("Error: El padre no registró la pieza.");
  });

  it("Actualiza la métrica del componente", async () => {
    await program.methods
      .actualizarComponente(modeloPieza, 2200, 2)
      .accounts({
        componente: componentePda,
        usuario: wallet.publicKey,
      })
      .rpc();

    let piezaData;
    let intentos = 0;
    do {
      piezaData = await program.account.componente.fetch(componentePda, "confirmed");
      intentos++;
    } while (piezaData.metricaPrincipal !== 2200 && intentos < 5);

    if (piezaData.metricaPrincipal !== 2200) throw new Error("Error: La métrica no se actualizó.");
    if (piezaData.estadoCalidad !== 2) throw new Error("Error: El estado de calidad no cambió.");
  });

  it("Elimina el componente y recupera SOL", async () => {
    await program.methods
      .eliminarComponente(modeloPieza)
      .accounts({
        componente: componentePda,
        usuario: wallet.publicKey,
      })
      .rpc();

    let cuentaBorrada = false;
    let intentos = 0;
    
    while (!cuentaBorrada && intentos < 5) {
      try {
        await program.account.componente.fetch(componentePda, "confirmed");
        intentos++;
      } catch (e) {
        cuentaBorrada = true;
      }
    }

    if (!cuentaBorrada) throw new Error("Error: La cuenta sigue viva en la red, no se eliminó.");
  });
});