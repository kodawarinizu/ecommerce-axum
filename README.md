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
│   └── admin/
│       └── dashboard.html
└── src/
    ├── main.rs
    ├── db.rs                  ← DataBase Conexion
    ├── models/                ← Rust Struct Represent data
    │   ├── mod.rs
    │   └── product.rs
    ├── repositories/          ← DB accest (queries)
    │   ├── mod.rs
    │   └── product_repository.rs
    ├── services/              ← Bussnes Logic
    │   ├── mod.rs
    │   └── product_service.rs
    └── handlers/              ← HTTP routes (controllers)
        ├── mod.rs
        ├── store.rs           ← Public store
        └── admin.rs           ← Admin panel
```

### Dependencies
```toml
[dependencies]
axum = "0.8.9"
dotenvy = "0.15.7"
serde = { version = "1.0.228", features = ["derive"] }
sqlx = { version = "0.8.6", features = ["postgres", "runtime-tokio", "uuid", "macros"] }
tera = "1.20.1"
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.6.8", features = ["fs"] }
tracing = "0.1.44"
tracing-subscriber = "0.3.23"
uuid = { version = "1.23.1", features = ["v4"] }
rust_decimal = { version = "1.41.0", features = ["macros"] }
```