# SCI All Stream API

Una API RESTful desarrollada en **Rust** utilizando el framework **Rocket**. Este proyecto está diseñado para manejar flujos de datos con un enfoque modular y eficiente.
## Tecnologías
- **Rust**: Un lenguaje de programación rápido y seguro.
- **Rocket**: Framework web para construir APIs rápidas y robustas.
- **Diesel**: ORM para interactuar con la base de datos.
## Requisitos previos

1. **Instalar Rust**  
   Asegúrate de tener Rust instalado. Puedes hacerlo desde su página oficial: [Rust](https://www.rust-lang.org/tools/install).

2. **Clonar el repositorio**  
   Clona este proyecto en tu máquina local:
   ```bash
   git clone https://github.com/Noctis-Dev/sci-all-stream-api.git
   cd sci-all-stream-api
Configurar la base de datos
Modifica el archivo diesel.toml y establece la conexión a tu base de datos. Además, asegúrate de haber creado las migraciones necesarias con Diesel:
bash
Copiar código
diesel setup
diesel migration run
Configurar variables de entorno
Crea un archivo .env en la raíz del proyecto con las configuraciones necesarias:
env
Copiar código
DATABASE_URL=postgres://usuario:contraseña@localhost/nombre_db
ROCKET_PORT=8000
Cómo ejecutar la API
Construir y ejecutar
Usa los siguientes comandos para construir y ejecutar el proyecto:
bash
Copiar código
cargo build
cargo run
Permisos y puertos
La API estará disponible por defecto en http://localhost:8000.
