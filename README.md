# API REST Rust

API REST em Rust com [Actix Web](https://actix.rs/) e [Diesel](https://diesel.rs/) (PostgreSQL).

## Estrutura

```
src/
├── main.rs                  # Ponto de entrada, HttpServer + pool + JWT
├── schema.rs                # Definição das tabelas (Diesel table!)
├── auth/
│   ├── mod.rs               # Declaração do módulo
│   ├── models.rs            # Claims JWT, LoginRequest/Response
│   ├── jwt.rs               # Criação e verificação de tokens
│   ├── extractor.rs         # Extractor AuthenticatedUser (FromRequest)
│   ├── routes.rs            # Endpoints de autenticação
│   └── service.rs           # Lógica de login (Argon2 + JWT)
├── libs/
│   └── connection.rs        # Conexão directa (legado)
├── models/
│   └── user.rs              # Modelos de domínio
├── repositories/
│   └── user_repository.rs   # Acesso a dados (queries SQL/Diesel)
├── services/
│   └── user_service.rs      # Lógica de negócio
└── routes/
    ├── routes.rs            # Handlers (endpoints)
    └── router.rs            # Registo centralizado de rotas
```

### Camadas

- **`auth/`** — Autenticação JWT. Models, criação/verificação de tokens, extractor para proteger rotas, serviço de login.
- **`routes/`** — Handlers HTTP. Extraem parâmetros do request e devolvem respostas.
- **`services/`** — Lógica de negócio. Orquestra chamadas aos repositórios.
- **`repositories/`** — Acesso a dados. Queries Diesel puras.
- **`models/`** — Estruturas de domínio (mapeamento tabela → struct).

## Pré-requisitos

- Rust (edition 2024)
- PostgreSQL (com `libpq` — `sudo apt install libpq-dev`)
- diesel_cli (`cargo install diesel_cli --no-default-features --features postgres`)

> Se o linker falhar com `unable to find library -lpq`, cria um symlink local:
> ```bash
> ln -s /usr/lib64/libpq.so.5 libpq.so
> ```
> O `.cargo/config.toml` já define `PQ_LIB_DIR` apontando para a raiz do projecto.

## Setup

```bash
# 1. Configurar variáveis de ambiente
cp .env.example .env
# editar DATABASE_URL e JWT_SECRET se necessário

# 2. Criar o banco (se não existir)
diesel database setup

# 3. Executar migrations
diesel migration run

# 4. Iniciar servidor
cargo run
```

### Variáveis de ambiente

| Variável       | Obrigatória | Descrição                          |
|----------------|-------------|------------------------------------|
| `DATABASE_URL` | sim         | URL de conexão PostgreSQL          |
| `JWT_SECRET`   | sim         | Chave secreta para assinar tokens JWT |

## Endpoints

| Método | Rota             | Autenticação        | Descrição                        |
|--------|------------------|---------------------|----------------------------------|
| GET    | `/`              | ❌                  | Health check                     |
| POST   | `/users`         | ❌                  | Criar novo user (cadastro)       |
| POST   | `/auth/login`    | ❌                  | Login → retorna JWT              |
| GET    | `/auth/me`       | ✅ Bearer Token     | Dados do usuário logado          |
| GET    | `/users`         | ✅ Bearer Token     | Listar users paginados           |

---

### POST /users

Cria um novo user. A password é encriptada com **Argon2** e o `id` é gerado automaticamente como UUID.

**Body:**

| Campo      | Tipo   | Obrigatório | Descrição                         |
|------------|--------|-------------|-----------------------------------|
| `name`     | string | sim         | Nome do user (max 100 caracteres) |
| `email`    | string | sim         | Email válido                      |
| `password` | string | sim         | Senha (mínimo 6 caracteres)       |

**Exemplo:**
```bash
curl -X POST "http://localhost:8080/users" \
  -H "Content-Type: application/json" \
  -d '{"name":"João Silva","email":"joao@email.com","password":"123456"}'
```

**Resposta sucesso (201):**
```json
{
  "id": "a8d0f4f7-5657-4be9-8367-74f553995efd",
  "name": "João Silva",
  "email": "joao@email.com",
  "created_at": "2026-07-06T13:49:21Z"
}
```

**Erro de validação (422):**
```json
{
  "errors": [
    {"field": "name", "message": "Nome é obrigatório"},
    {"field": "email", "message": "Email inválido"},
    {"field": "password", "message": "Senha deve ter no mínimo 6 caracteres"}
  ]
}
```

---

### POST /auth/login

Autentica o user e retorna um token JWT (válido por 24h).

**Body:**

| Campo      | Tipo   | Obrigatório | Descrição                    |
|------------|--------|-------------|------------------------------|
| `email`    | string | sim         | Email do user                |
| `password` | string | sim         | Senha do user                |

**Exemplo:**
```bash
curl -X POST "http://localhost:8080/auth/login" \
  -H "Content-Type: application/json" \
  -d '{"email":"joao@email.com","password":"123456"}'
```

**Resposta sucesso (200):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhOGQwZjRmNy01NjU3LTRiZTktODM2Ny03NGY1NTM5OTVlZmQiLCJlbWFpbCI6ImpvYW9AZW1haWwuY29tIiwiZXhwIjoxNzUwOTUyMDAwfQ.abc123",
  "user_id": "a8d0f4f7-5657-4be9-8367-74f553995efd",
  "email": "joao@email.com"
}
```

**Erro (401):**
```json
{
  "error": "Email ou senha inválidos"
}
```

> O token deve ser enviado no header `Authorization: Bearer <token>` nas rotas privadas.

---

### GET /auth/me

Retorna os dados do user autenticado com base no token JWT.

**Requires:** `Authorization: Bearer <token>`

**Exemplo:**
```bash
curl "http://localhost:8080/auth/me" \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiJ9.abc123"
```

**Resposta sucesso (200):**
```json
{
  "id": "a8d0f4f7-5657-4be9-8367-74f553995efd",
  "name": "João Silva",
  "email": "joao@email.com",
  "created_at": "2026-07-06T13:49:21Z"
}
```

**Erro (401):**
```json
{
  "error": "Token inválido: ..."
}
```

---

### GET /users (privada)

Lista users com paginação. Requer autenticação.

**Requires:** `Authorization: Bearer <token>`

**Query params:**

| Parâmetro  | Tipo  | Padrão | Descrição                |
|------------|-------|--------|--------------------------|
| `page`     | int   | 1      | Número da página         |
| `per_page` | int   | 10     | Itens por página (1-100) |

**Exemplo:**
```bash
curl "http://localhost:8080/users?page=1&per_page=20" \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiJ9.abc123"
```

**Resposta:**
```json
{
  "data": [],
  "page": 1,
  "per_page": 20,
  "total": 0,
  "total_pages": 0
}
```

---

### Fluxo completo de autenticação

```bash
# 1. Criar conta
curl -X POST "http://localhost:8080/users" \
  -H "Content-Type: application/json" \
  -d '{"name":"Maria","email":"maria@email.com","password":"123456"}'

# 2. Fazer login e guardar o token
TOKEN=$(curl -s -X POST "http://localhost:8080/auth/login" \
  -H "Content-Type: application/json" \
  -d '{"email":"maria@email.com","password":"123456"}' | jq -r '.token')

# 3. Aceder a rotas privadas
curl "http://localhost:8080/users" -H "Authorization: Bearer $TOKEN"
curl "http://localhost:8080/auth/me" -H "Authorization: Bearer $TOKEN"
```

## Migrations

```bash
# Criar nova migration
diesel migration generate <nome>

# Executar pendentes
diesel migration run

# Reverter última
diesel migration redo

# Re-gerar schema.rs a partir do banco
diesel print-schema > src/schema.rs
```
