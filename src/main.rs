pub mod errors;
pub mod domain;
pub mod application;
pub mod infrastructure;

use dotenvy::dotenv;
use crate::{errors::AppError, infrastructure::db};

#[tokio::main]
async fn main() {
    dotenv().map_err(|e| AppError::EnvError(e.to_string()));
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL no definida o credenciales incorrectas");

    let pool = db::connect(&database_url).await;
}
