# Usa una imagen base de Windows
FROM mcr.microsoft.com/windows/servercore:ltsc2019

# Copia el archivo .exe al contenedor
COPY ./target/release/sci-all-streams-api.exe /app/sci-all-streams-api.exe

# Establece el directorio de trabajo
WORKDIR /app

# Ejecuta el archivo .exe
CMD ["sci-all-streams-api.exe"]