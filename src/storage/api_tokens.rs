use crate::models::APIToken;
use crate::storage::Db;

impl Db {
    pub async fn get_api_token(&self, org: &str, encrypted_token: &str) -> APIToken {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query_as::<_, APIToken>(
            r#"
        SELECT encrypted_token, created, duration, org_id
        FROM api_tokens
        WHERE org_id = ? AND encrypted_token = ?
            "#,
        )
        .bind(org)
        .bind(encrypted_token)
        .fetch_one(&mut conn)
        .await
        .unwrap()
    }

    pub async fn create_api_token(&self, token: &APIToken) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        INSERT INTO api_tokens (encrypted_token, created, duration, org_id)
        VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(&token.encrypted_token)
        .bind(token.created)
        .bind(token.duration)
        .bind(&token.org_id)
        .execute(&mut conn)
        .await
        .unwrap();
    }

    pub async fn delete_api_token(&self, org_id: &str, token: &str) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        DELETE FROM api_tokens
        WHERE org_id = ? AND encrypted_token = ?
            "#,
        )
        .bind(&org_id)
        .bind(&token)
        .execute(&mut conn)
        .await
        .unwrap();
    }
}
