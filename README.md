### Node Hardware Registry 

Una base de datos relacional descentralizada construida en la blockchain de Solana para la gestión y auditoría de componentes de hardware en nodos de trabajo.

Este proyecto fue desarrollado como parte del Solana LATAM Builders Program, implementando arquitectura de Program Derived Addresses (PDAs) para garantizar el almacenamiento determinista y la inmutabilidad de los registros.

### Arquitectura Técnica

El protocolo funciona bajo un modelo relacional de Padre-Hijo (1:N):

    WorkstationDB (Padre): Cuenta inicializada por la wallet administradora que actúa como el nodo principal (ej. Master-Forge). Almacena el nombre del equipo y un Vector de llaves públicas (Pubkeys) que apuntan a sus componentes.

    Componente (Hijo): Cuentas individuales derivadas de la wallet y el modelo específico del hardware. Almacenan métricas de rendimiento y estado de calidad.

## Stack Tecnológico

    **Smart Contract:** Rust + Anchor Framework.

    **Testing & Telemetría:** TypeScript, Mocha, Chai.

    **Red de Despliegue:** Solana Devnet.
    
### Características Principales (CRUD)

    **Create (Creación Determinista):** Inicialización de bases de datos de Workstations y adición de componentes únicos evadiendo colisiones de memoria mediante inyección de entropía.

    **Read (Lectura con Polling):** Sistema de pruebas asíncronas con ciclos de reintentos (Polling) para mitigar la latencia de lectura natural en clústeres descentralizados.

    **Update (Actualización de Estado):** Modificación inmutable de métricas de rendimiento y estatus de calidad de las piezas registradas.

    **Delete (Recuperación de Renta):** Cierre de cuentas de componentes individuales y recuperación exitosa de la renta (SOL) hacia la wallet administradora, manteniendo la eficiencia del almacenamiento on-chain.

### Instrucciones de Uso (Entorno Local)

## Requisitos Previos

    Node.js y Yarn instalados.

    Solana CLI configurado.

    Anchor CLI instalado.

### Instalación y Pruebas

Paso 1. Clona este repositorio:
git clone https://github.com/Carlos-DataArch/Node-Hardware-Registry.git

Paso 2. Instala las dependencias del cliente:
yarn install

Paso 3. Ejecuta la batería de pruebas de integración (Asegúrate de tener SOL de prueba en tu wallet local):
anchor test

## Guía de Interacción y Despliegue

Opción A: Interacción Manual (Solana Playground)

    Build & Deploy: Ejecute el comando Build y posteriormente Deploy en el clúster de Devnet.

    Sincronización de Interfaz: En la pestaña "Build & Deploy" (ícono de martillo y llave), haga clic en Initialize dentro de la sección IDL.

    Registro de Hardware:

        Una vez inicializado, refresque el navegador (F5).

        Expanda la sección Instructions en el panel lateral.

        Use crearWorkstationDb para registrar el nodo principal.

        Use agregarComponente para vincular hardware específico.

Opción B: Validación en Entorno Local (CLI)

Para desarrolladores con el Toolchain de Solana y Anchor instalado localmente:

    Instale dependencias de Node.js:
    npm install o yarn install

    Ejecute la suite de pruebas de integración:
    anchor test
    (Nota: El script incluye lógica de reintentos asíncronos para compensar la latencia de confirmación en Devnet).
