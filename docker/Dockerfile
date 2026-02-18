# Imagen base con Rust
FROM rust:latest AS builder

WORKDIR /usr/src/app

# Copiar proyecto Rust desde /rust

COPY rust .

# Compilar en modo release 

RUN cargo build --release



FROM rust:latest

WORKDIR /usr/src/app

# Copiar solo el binario compilado

COPY --from=builder /usr/src/app/target/release/algoritmo_distribuido .

# Comando por defecto

CMD ["./algoritmo_distribuido"]
