# Step 1: Image for development
FROM node:lts AS development

# Set the working directory
WORKDIR /app

# Copy necessary files
COPY package.json package-lock.json ./

# Install dependencies
RUN npm install

# Copy the entire project
COPY . .

# Expose port 5173 for the development server
EXPOSE 5173

# Start the Vite server in development mode
CMD ["npm", "run", "dev"]

# Step 2: Build for production
FROM node:lts AS builder

WORKDIR /app
COPY package.json package-lock.json ./
RUN npm install
COPY . .
RUN npm run build

# Step 3: Final image with NGINX to serve static files
FROM nginx:stable-alpine

# Copy built files
COPY --from=builder /app/dist /usr/share/nginx/html

# Expose port for production
EXPOSE 80

# Start NGINX
CMD ["nginx", "-g", "daemon off;"]
