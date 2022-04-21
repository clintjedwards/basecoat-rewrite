use crate::models::Base;
use crate::storage::Db;

impl Db {
    pub async fn list_bases(&self, org: &str) -> Vec<Base> {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query_as::<_, Base>(
            r#"
        SELECT id, name, manufacturer, created, modified, org_id
        FROM bases
        WHERE org_id = ?
        ORDER BY name
            "#,
        )
        .bind(org)
        .fetch_all(&mut conn)
        .await
        .unwrap()
    }

    pub async fn create_base(&self, base: &Base) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        INSERT INTO bases (id, name, manufacturer, created, modified, org_id)
        VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&base.id)
        .bind(&base.name)
        .bind(&base.manufacturer)
        .bind(base.created)
        .bind(base.modified)
        .bind(&base.org_id)
        .execute(&mut conn)
        .await
        .unwrap();
    }

    pub async fn get_base(&self, org: &str, id: &str) -> Base {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query_as::<_, Base>(
            r#"
        SELECT id, name, manufacturer, created, modified, org_id
        FROM bases
        WHERE org_id = ? AND id = ?
            "#,
        )
        .bind(org)
        .bind(id)
        .fetch_one(&mut conn)
        .await
        .unwrap()
    }

    pub async fn delete_base(&self, org: &str, id: &str) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        DELETE FROM bases
        WHERE org_id = ? AND id = ?
            "#,
        )
        .bind(org)
        .bind(id)
        .execute(&mut conn)
        .await
        .unwrap();
    }
}
