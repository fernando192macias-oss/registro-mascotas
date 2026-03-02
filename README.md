
🏋️🐶 PetChain — Gestión de Mascotas en Solana
PetChain es un programa on-chain desarrollado en Rust con Anchor sobre la blockchain de Solana. Permite a dueños de mascotas o clínicas veterinarias gestionar mascotas y sus registros de forma descentralizada, transparente e inmutable.

📌 ¿Qué hace el proyecto?
PetChain implementa un sistema CRUD completo para administrar mascotas:

Crear un perfil de dueño vinculado a tu wallet (owner)
Registrar mascotas con nombre, especie y edad
Eliminar mascotas cerrando su cuenta en la blockchain
Activar/desactivar estado (ej: mascota en tratamiento)
Actualizar datos como edad o estado de salud

Cada dueño y cada mascota son cuentas derivadas (PDA) únicas en Solana, garantizando:

No puede haber duplicados
Solo el owner autorizado puede modificar su información


🏗️ Arquitectura
Owner (Wallet)
    │
    └── Dueño (PDA)
            │
            ├── Mascota A (PDA)
            ├── Mascota B (PDA)
            └── Mascota C (PDA)

📦 Structs principales
🧑 Dueño
CampoTipoDescripciónownerPubkeyWallet del dueñonombreStringNombre del dueñomascotasVec<Pubkey>Lista de PDAs de mascotas
🐾 Mascota
CampoTipoDescripciónduenoStringNombre del dueñonombreStringNombre de la mascotaespecieStringPerro, gato, ave, etc.edadu8Edad en añosactivaboolEstado (activa / inactiva)

⚙️ Instrucciones (Funciones del programa)
InstrucciónDescripcióncrear_dueno(nombre)Crea la cuenta del dueño vinculada al ownerregistrar_mascota(nombre, especie, edad)Registra una nueva mascotaeliminar_mascota(nombre)Elimina la mascota y cierra su cuentaalternar_estado(nombre)Activa o desactiva la mascotaactualizar_edad(nombre, edad)Actualiza la edad de la mascota

🔐 PDAs (Program Derived Addresses)
Las cuentas se derivan con los siguientes seeds:
CuentaSeedsDueño["dueno", nombre_dueno, owner_pubkey]Mascota["mascota", nombre_mascota, owner_pubkey]
Esto garantiza que:

Cada wallet puede tener su propio perfil único
No pueden existir dos mascotas con el mismo nombre bajo el mismo dueño
Solo el owner puede modificar sus mascotas


🚀 Cómo usar el proyecto (Solana Playground)

Abre Solana Playground
Haz fork del repositorio o pega el contenido en src/lib.rs
Conecta tu wallet (devnet)
Haz clic en Build y luego Deploy
Usa el panel de Test para interactuar con el programa

📌 Ejemplo de flujo
bash1. crear_dueno("Fernando")
2. registrar_mascota("Firulais", "Perro", 3)
3. alternar_estado("Firulais")     → cambia estado
4. actualizar_edad("Firulais", 4)  → cumple años
5. eliminar_mascota("Firulais")    → elimina registro

🛠️ Tecnologías
TecnologíaDescripciónSolanaBlockchain de alta velocidadAnchor FrameworkFramework para programas Solana en RustRustLenguaje principal del programa

👤 Autor
Proyecto desarrollado como parte de la certificación de Solana, adaptado al modelo de gestión de mascotas.
