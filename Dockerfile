# Utiliser la dernière version LTS de l'image Rust
FROM rust:1.65 as builder

# Installer des dépendances pour optimiser les performances
RUN apt-get update && apt-get install -y \
    cmake \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Définir le répertoire de travail
WORKDIR /usr/src/app

# Copier les fichiers Cargo.toml et Cargo.lock depuis le répertoire de build (le contexte)
COPY ./Cargo.toml ./Cargo.lock ./

# Préparer les dépendances (cela permet de ne pas les recompiler si elles ne changent pas)
RUN cargo build --release
RUN rm -rf src/

# Copier tout le code source de l'application
COPY . .

# Construire l'application en mode release
RUN cargo install --path .

# Étape finale : création d'une image plus légère pour exécuter l'application
FROM debian:buster-slim

# Installer les dépendances nécessaires à l'exécution
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copier l'application compilée depuis l'étape de construction
COPY --from=builder /usr/local/cargo/bin/my_rust_app /usr/local/bin/my_rust_app

# Exposer le port sur lequel l'application écoute
EXPOSE 8080

# Définir la commande de démarrage
CMD ["my_rust_app"]
