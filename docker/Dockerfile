# Etapa de compilación
FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY . .

WORKDIR /usr/src/app/rust

RUN cargo build --release


# Imagen final
FROM debian:bookworm-slim

WORKDIR /usr/src/app

# Instalar herramientas de red
RUN apt-get update && \
    apt-get install -y iproute2 iputils-ping && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/rust/target/release/algoritmo_distribuido .

CMD ["./algoritmo_distribuido"]
