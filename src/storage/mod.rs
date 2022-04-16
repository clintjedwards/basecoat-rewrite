use sqlx::{Pool, Sqlite, SqlitePool};
use std::error::Error;

#[derive(Default, Clone)]
pub struct Db {
    conn: Option<Pool<Sqlite>>,
}

impl Db {
    pub async fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let connection_pool = SqlitePool::connect(&format!("file:{}", path))
            .await
            .unwrap();
        let db = Db {
            conn: Some(connection_pool),
        };

        db.create_tables().await;

        Ok(db)
    }

    async fn create_tables(&self) {
        sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS accounts (
            id       TEXT    PRIMARY KEY,
            name     TEXT    NOT NULL,
            hash     TEXT    NOT NULL,
            state    TEXT    NOT NULL,
            created  INTEGER NOT NULL,
            modified INTEGER NOT NULL
        );
            "#,
        )
        .execute(self.conn.as_ref().unwrap())
        .await
        .unwrap();
    }

    // pub async fn create_account(&self) {
    //     sqlx::query(
    //         "
    //     Select * From weow
    //         ",
    //     )
    //     .execute(self.conn.as_ref().unwrap())
    //     .await;
    // }
}
