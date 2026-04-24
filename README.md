# Node Hardware Registry

Una base de datos relacional descentralizada construida en la blockchain de Solana para la gestión y auditoría de componentes de hardware en nodos de trabajo.

Este proyecto fue desarrollado como parte del Solana LATAM Builders Program, implementando una arquitectura de Program Derived Addresses (PDAs) para garantizar el almacenamiento determinista y la inmutabilidad de los registros.

## Arquitectura Técnica

El protocolo funciona bajo un modelo relacional de Padre-Hijo (1:N):

* **WorkstationDB (Padre):** Cuenta inicializada por la wallet administradora que actúa como el nodo principal (ej. Master-Forge). Almacena el nombre del equipo y un Vector de llaves públicas (Pubkeys) que apuntan a sus componentes.
* **Componente (Hijo):** Cuentas individuales derivadas de la wallet y el modelo específico del hardware. Almacenan métricas de rendimiento y estado de calidad.

## Stack Tecnológico

* **Smart Contract:** Rust + Anchor Framework.
* **Testing & Telemetría:** TypeScript, Mocha, Chai.
* **Red de Despliegue:** Solana Devnet.

## Características Principales (CRUD)

1. **Create (Creación Determinista):** Inicialización de bases de datos de Workstations y adición de componentes únicos evadiendo colisiones de memoria mediante inyección de entropía.
2. **Read (Lectura con Polling):** Sistema de pruebas asíncronas con ciclos de reintentos (Polling) para mitigar la latencia de lectura natural en clústeres descentralizados.
3. **Update (Actualización de Estado):** Modificación inmutable de métricas de rendimiento y estatus de calidad de las piezas registradas.
4. **Delete (Recuperación de Renta):** Cierre de cuentas de componentes individuales y recuperación exitosa de la renta (SOL) hacia la wallet administradora, manteniendo la eficiencia del almacenamiento on-chain.

## Guía de Interacción y Despliegue

### Opción A: Interacción Manual (Solana Playground)

1. **Build & Deploy:** Ejecute el comando `Build` y posteriormente `Deploy` en el clúster de Devnet.
2. **Sincronización de Interfaz:** En la pestaña de herramientas, haga clic en `Initialize` dentro de la sección **IDL**.
3. **Registro de Hardware:**
    * Una vez inicializado, refresque el navegador (F5).
    * Expanda la sección **Instructions** en el panel lateral.
    * Use `crearWorkstationDb` para registrar el nodo principal.
    * Use `agregarComponente` para vincular hardware específico.

### Opción B: Validación en Entorno Local (CLI)

Para desarrolladores con el Toolchain de Solana y Anchor instalado localmente:

1. **Instalación:**
    ```bash
    npm install
    ```
2. **Pruebas:**
    ```bash
    anchor test
    ```
    *Nota: El script incluye lógica de reintentos asíncronos para compensar la latencia de confirmación en Devnet.*
