# Utiliser l'image Rust comme base
FROM rust:buster AS base

# Définir les variables d'environnement nécessaires pour l'application
ENV USER=root
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=development

# Créer un répertoire de travail
WORKDIR /code

# Définir les variables d'environnement pour libclang et pkg-config
ENV LIBCLANG_PATH=/usr/lib/llvm-12/lib
ENV LD_LIBRARY_PATH=/usr/lib/llvm-12/lib
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

# Installer les dépendances pour Rust, FFmpeg, et Clang
RUN apt-get update && apt-get install -y \
  cmake \
  pkg-config \
  libssl-dev \
  ffmpeg \
  libavfilter-dev \
  libavformat-dev \
  libavcodec-dev \
  libavutil-dev \
  libavdevice-dev \
  clang \
  libclang-dev \
  llvm-dev \
  && rm -rf /var/lib/apt/lists/*

# Initialiser le projet Cargo
RUN cargo init

# Copier les fichiers Cargo.toml et Cargo.lock
COPY ./Cargo.toml ./Cargo.lock ./ 

# Télécharger les dépendances Cargo
RUN cargo fetch

# Copier le reste du code source
COPY . /code

# Phase de développement avec l'exécution du projet en mode offline
FROM base AS development

# Exposer le port 8000 pour l'application
EXPOSE 8000

# Commande pour exécuter l'application en mode développement
CMD [ "cargo", "run" ]

# Phase pour des environnements de développement avancés
FROM base AS dev-envs

# Exposer le port 8000 pour l'application
EXPOSE 8000

# Installer git et autres outils de développement si nécessaire
RUN <<EOF
apt-get update
apt-get install -y --no-install-recommends git
EOF

# Créer un utilisateur vscode avec des droits Docker
RUN <<EOF
useradd -s /bin/bash -m vscode
groupadd docker
usermod -aG docker vscode
EOF

# Copier Docker CLI et d'autres outils de build si nécessaire
COPY --from=gloursdocker/docker / /

# Commande pour exécuter l'application en mode développement
CMD [ "cargo", "run" ]

# Phase de construction en release pour la production
FROM base AS builder

# Construire l'application en mode release
RUN cargo build --release

# Phase finale : Utiliser une image plus légère pour exécuter l'application
FROM debian:buster-slim

# Définir les variables d'environnement pour la production
ENV ROCKET_ENV=production
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

# Installer les dépendances nécessaires pour l'exécution de l'application
RUN apt-get update && apt-get install -y \
  ca-certificates \
  ffmpeg \
  && rm -rf /var/lib/apt/lists/*

# Copier l'application compilée depuis l'étape de build
COPY --from=builder /code/target/release/react-rust-postgres /react-rust-postgres

# Exposer le port 8000 pour l'application en production
EXPOSE 8000

# Commande pour exécuter l'application en production
CMD [ "/react-rust-postgres" ]
