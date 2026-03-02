🏋️🐶 PetChain — Gestión de Mascotas en Solana

PetChain es un programa on-chain desarrollado en Rust con Anchor sobre la blockchain de Solana.
Permite a dueños de mascotas (o clínicas veterinarias) gestionar mascotas y sus registros de forma descentralizada, transparente e inmutable.

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
Campo	Tipo	Descripción
owner	Pubkey	Wallet del dueño
nombre	String	Nombre del dueño
mascotas	Vec<Pubkey>	Lista de PDAs de mascotas
🐾 Mascota
Campo	Tipo	Descripción
dueno	String	Nombre del dueño
nombre	String	Nombre de la mascota
especie	String	Perro, gato, ave, etc
edad	u8	Edad en años
activa	bool	Estado (activa / inactiva)
⚙️ Instrucciones (Funciones del programa)
Instrucción	Descripción
crear_dueno(nombre)	Crea la cuenta del dueño vinculada al owner
registrar_mascota(nombre, especie, edad)	Registra una nueva mascota
eliminar_mascota(nombre)	Elimina la mascota y cierra su cuenta
alternar_estado(nombre)	Activa o desactiva la mascota
actualizar_edad(nombre, edad)	Actualiza la edad de la mascota
🔐 PDAs (Program Derived Addresses)

Las cuentas se derivan con los siguientes seeds:

Dueño:
["dueno", nombre_dueno, owner_pubkey]

Mascota:
["mascota", nombre_mascota, owner_pubkey]

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

📌 Ejemplo de flujo:
1. crear_dueno("Fernando")
2. registrar_mascota("Firulais", "Perro", 3)
3. alternar_estado("Firulais")    → cambia estado
4. actualizar_edad("Firulais", 4) → cumple años
5. eliminar_mascota("Firulais")   → elimina registro
🛠️ Tecnologías

Solana — Blockchain de alta velocidad

Anchor Framework — Framework para programas Solana en Rust

Rust — Lenguaje del programa

👤 Autor

Proyecto desarrollado como parte de la certificación de Solana, adaptado al modelo de gestión de mascotas.
