use crate::models::Account;
use crate::storage::Db;

impl Db {
    pub async fn create_account(&self, account: Account) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        INSERT INTO accounts (id, name, hash, state, created, modified)
        VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(account.id)
        .bind(account.name)
        .bind(account.hash)
        .bind(account.state)
        .bind(account.created)
        .bind(account.modified)
        .execute(&mut conn)
        .await
        .unwrap();
    }
}
