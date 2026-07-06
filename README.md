# API REST Rust

API REST em Rust com [Actix Web](https://actix.rs/) e [Diesel](https://diesel.rs/) (PostgreSQL).

## Estrutura

```
src/
├── main.rs                  # Ponto de entrada, HttpServer + pool de conexões
├── schema.rs                # Definição das tabelas (Diesel table!)
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
# editar DATABASE_URL se necessário

# 2. Criar o banco (se não existir)
diesel database setup

# 3. Executar migrations
diesel migration run

# 4. Iniciar servidor
cargo run
```

## Endpoints

| Método | Rota             | Descrição                        |
|--------|------------------|----------------------------------|
| GET    | `/`              | Health check                     |
| GET    | `/users`         | Listar users paginados           |
| POST   | `/users`         | Criar novo user                  |

### GET /users

Query params:

| Parâmetro  | Tipo  | Padrão | Descrição                |
|------------|-------|--------|--------------------------|
| `page`     | int   | 1      | Número da página         |
| `per_page` | int   | 10     | Itens por página (1-100) |

Exemplo:
```bash
curl "http://localhost:8080/users?page=1&per_page=20"
```

Resposta:
```json
{
  "data": [],
  "page": 1,
  "per_page": 20,
  "total": 0,
  "total_pages": 0
}
```

### POST /users

Cria um novo user. A password é encriptada com **Argon2** e o `id` é gerado automaticamente como UUID.

Body:

| Campo      | Tipo   | Obrigatório | Descrição                         |
|------------|--------|-------------|-----------------------------------|
| `name`     | string | sim         | Nome do user (max 100 caracteres) |
| `email`    | string | sim         | Email válido                      |
| `password` | string | sim         | Senha (mínimo 6 caracteres)       |

Exemplo:
```bash
curl -X POST "http://localhost:8080/users" \
  -H "Content-Type: application/json" \
  -d '{"name":"João Silva","email":"joao@email.com","password":"123456"}'
```

Resposta sucesso (201):
```json
{
  "id": "a8d0f4f7-5657-4be9-8367-74f553995efd",
  "name": "João Silva",
  "email": "joao@email.com",
  "created_at": "2026-07-06T13:49:21Z"
}
```

Erro de validação (422):
```json
{
  "errors": [
    {"field": "name", "message": "Nome é obrigatório"},
    {"field": "email", "message": "Email inválido"},
    {"field": "password", "message": "Senha deve ter no mínimo 6 caracteres"}
  ]
}
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
