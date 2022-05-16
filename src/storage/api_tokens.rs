use crate::models::APIToken;
use crate::storage::Db;

impl Db {
    pub async fn get_api_token(&self, org: &str, hash: &str) -> APIToken {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query_as::<_, APIToken>(
            r#"
        SELECT hash, created, duration, org_id
        FROM api_tokens
        WHERE org_id = ? AND hash = ?
            "#,
        )
        .bind(org)
        .bind(hash)
        .fetch_one(&mut conn)
        .await
        .unwrap()
    }

    pub async fn create_api_token(&self, token: &APIToken) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        INSERT INTO api_tokens (hash, created, duration, org_id)
        VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(&token.hash)
        .bind(token.created)
        .bind(token.duration)
        .bind(&token.org_id)
        .execute(&mut conn)
        .await
        .unwrap();
    }

    pub async fn delete_api_token(&self, token: &APIToken) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        INSERT INTO api_tokens (hash, created, duration, org_id)
        VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(&token.hash)
        .bind(token.created)
        .bind(token.duration)
        .bind(&token.org_id)
        .execute(&mut conn)
        .await
        .unwrap();
    }
}
