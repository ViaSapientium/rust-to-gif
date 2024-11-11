# Étape 1 : Image pour le développement
FROM node:lts AS development

# Définir le dossier de travail
WORKDIR /app

# Copier les fichiers nécessaires
COPY package.json package-lock.json ./

# Installer les dépendances
RUN npm install

# Copier tout le projet
COPY . .

# Exposer le port 5173 pour le serveur de développement
EXPOSE 5173

# Lancer le serveur Vite en mode développement
CMD ["npm", "run", "dev"]

# Étape 2 : Construire pour la production
FROM node:lts AS builder

WORKDIR /app
COPY package.json package-lock.json ./
RUN npm install
COPY . .
RUN npm run build

# Étape 3 : Image finale avec NGINX pour servir les fichiers statiques
FROM nginx:stable-alpine

# Copier les fichiers construits
COPY --from=builder /app/dist /usr/share/nginx/html

# Exposer le port pour la prod
EXPOSE 80

# Lancer NGINX
CMD ["nginx", "-g", "daemon off;"]
