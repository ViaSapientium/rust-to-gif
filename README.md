## Installation and Execution

### Prerequisites

- **Rust** (stable version)
- **PostgreSQL** installed and configured
- **FFmpeg** for video management

### Installation

1. Clone this repository:

   ```bash
   git clone https://github.com/your-repository/rust-to-gif.git
   ```

2. Navigate to the project directory:

   ```bash
   cd rust-to-gif-backend
   ```

3. Create a `.env` file in the root directory and add the following environment variables:

   ```env
    DATABASE_URL=postgres://username:password@localhost/dbname
    SECRET_KEY=your_secret_key
   ```

4. Install the required dependencies: Ensure that Rust is installed. You can install Rust by following instructions from rust-lang.org. Then, in the project directory, run:

   ```bash
   cargo build
   ```

5. Run the project:

   ```bash
   cargo run
   ```

6. Access the API at `http://localhost:8080`.
7. You can test the API using tools like **Postman** or **curl**.

# Commandes utiles postgres

### Lister les tables de la base de données

```bash
\dt
```

### Afficher les bases de données existantes

```bash
\l
```

### Afficher la structure d'une table

```bash
\d nom_de_la_table
```

### Exécuter une requête SQL

```bash
SELECT * FROM nom_de_la_table;
```

### Quitter la session PostgreSQL

```bash
\q
```

### Créer une base de données

```bash
CREATE DATABASE nom_de_la_base;
```

### Supprimer une base de données

```bash
DROP DATABASE nom_de_la_base;
```

### Créer un utilisateur

```bash
CREATE USER nom_utilisateur WITH PASSWORD
'mot_de_passe';
```

### Supprimer un utilisateur

```bash
DROP USER nom_utilisateur;
```

### Donner les droits à un utilisateur sur une base de données

```bash
GRANT ALL PRIVILEGES ON DATABASE nom_base TO nom_utilisateur;
```

### Supprimer les droits d'un utilisateur sur une base de données

```bash
REVOKE ALL PRIVILEGES ON DATABASE nom_base FROM nom_utilisateur;
```

### Donner les droits à un utilisateur sur toutes les tables d'une base de données

```bash
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO nom_utilisateur;
```

### Supprimer les droits d'un utilisateur sur toutes les tables d'une base de données

```bash
REVOKE ALL PRIVILEGES ON ALL TABLES IN SCHEMA public FROM nom_utilisateur;
```

### Donner les droits à un utilisateur sur une table

```bash
GRANT ALL PRIVILEGES ON nom_table TO nom_utilisateur;
```

### Supprimer les droits d'un utilisateur sur une table

```bash
REVOKE ALL PRIVILEGES ON nom_table FROM nom_utilisateur;
```
