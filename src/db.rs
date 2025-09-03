use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};

pub const DB_URL: &str = "sqlite://sqlite.db";

pub async fn init_db() -> SqlitePool {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        Sqlite::create_database(DB_URL).await.unwrap();
    }

    let pool = SqlitePool::connect(DB_URL).await.unwrap();

    let sql = std::fs::read_to_string("migrations/001_create_tasks_table.sql")
        .expect("Failed to read migration file");
    sqlx::query(&sql).execute(&pool).await.unwrap();

    pool
}
