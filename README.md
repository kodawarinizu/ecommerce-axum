# Ecommerce write in rust with axum 

### Arquitecture Layer
```
ecommerce/
├── Cargo.toml
├── .env
├── templates/
│   ├── base.html
│   ├── index.html
│   ├── products.html
│   ├── product_detail.html
│   ├── cart.html
│   ├── checkout.html
│   ├── auth/
│   │   ├── login.html
│   │   └── register.html
│   └── admin/
│       ├── base_admin.html
│       ├── dashboard.html
│       ├── products.html
│       └── orders.html
└── src/
    ├── main.rs
    ├── errors.rs                        ← AppError (HTTP)
    ├── domain/
    │   ├── mod.rs
    │   ├── errors.rs                    ← DomainError
    │   ├── entities/
    │   │   ├── mod.rs
    │   │   ├── product.rs
    │   │   ├── category.rs
    │   │   ├── user.rs
    │   │   └── order.rs
    │   ├── value_objects/
    │   │   ├── mod.rs
    │   │   ├── price.rs
    │   │   ├── email.rs
    │   │   └── password.rs
    │   └── ports/
    │       ├── mod.rs
    │       ├── product_repository.rs
    │       ├── category_repository.rs
    │       ├── user_repository.rs
    │       └── order_repository.rs
    ├── application/
    │   ├── mod.rs
    │   ├── product_service.rs
    │   ├── category_service.rs
    │   ├── order_service.rs
    │   └── auth/
    │       ├── mod.rs
    │       ├── admin_auth_service.rs    ← JWT httpOnly
    │       └── oauth_service.rs        ← Google OAuth
    └── infrastructure/
        ├── mod.rs
        ├── persistence/
        │   ├── mod.rs
        │   ├── postgres_product_repo.rs
        │   ├── postgres_category_repo.rs
        │   ├── postgres_user_repo.rs
        │   └── postgres_order_repo.rs
        ├── external/
        │   ├── mod.rs
        │   ├── mercadopago.rs           ← adapter MercadoPago
        │   └── google_oauth.rs          ← adapter Google OAuth
        └── http/
            ├── mod.rs
            ├── routes.rs
            ├── middleware/
            │   ├── mod.rs
            │   ├── auth_admin.rs        ← middleware JWT
            │   └── auth_user.rs         ← middleware OAuth
            └── handlers/
                ├── mod.rs
                ├── store.rs
                ├── cart.rs
                ├── checkout.rs
                └── admin.rs
```

### Dependencies
```toml
[dependencies]
argon2 = "0.5.3"
async-trait = "0.1.89"
axum = "0.8.9"
axum-extra = { version = "0.12.6", features = ["cookie"] }
dotenvy = "0.15.7"
jsonwebtoken = "10.3.0"
reqwest = { version = "0.13.3", features = ["json"] }
rust_decimal = { version = "1.41.0", features = ["macros", "serde"] }
serde = { version = "1.0.228", features = ["derive"] }
sqlx = { version = "0.8.6", features = ["postgres", "runtime-tokio", "uuid", "macros"] }
tera = "1.20.1"
thiserror = "2.0.18"
tokio = { version = "1", features = ["full"] }
tower = { version = "0.5.3", features = ["util"] }
tower-http = { version = "0.6.8", features = ["fs"] }
tracing = "0.1.44"
tracing-subscriber = "0.3.23"
uuid = { version = "1.23.1", features = ["v4", "serde"] }

```