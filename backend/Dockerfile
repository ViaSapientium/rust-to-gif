# Étape 1 : Construire le binaire Rust
FROM rust:1.79 as builder

# Installer les bibliothèques nécessaires
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq-dev \
    libclang-dev \
    clang \
    libavutil-dev \
    libavcodec-dev \
    libavformat-dev \
    libavdevice-dev \
    libavfilter-dev \
    pkg-config

# Configurer PKG_CONFIG_PATH pour FFmpeg
ENV PKG_CONFIG_PATH=/usr/lib/pkgconfig:/usr/local/lib/pkgconfig

# Définir le dossier de travail
WORKDIR /usr/src/app

# Copier Cargo.toml et Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Télécharger les dépendances
RUN cargo fetch

# Copier les fichiers sources
COPY . .

# Compiler le projet
RUN cargo build --release

# Étape 2 : Image légère pour exécuter le backend
FROM debian:buster-slim

# Installer les bibliothèques nécessaires
RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev

# Configurer le dossier de travail
WORKDIR /usr/src/app

# Copier le binaire depuis l'étape de build
COPY --from=builder /usr/src/app/target/release/rust_to_gif_backend .

# Exposer le port
EXPOSE 8000

# Démarrer le serveur
CMD ["./rust_to_gif_backend"]
