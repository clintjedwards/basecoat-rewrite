use crate::models::Organization;
use crate::storage::Db;

impl Db {
    pub async fn create_organization(&self, org: &Organization) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        INSERT INTO organizations (id, name, created, modified)
        VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(&org.id)
        .bind(&org.name)
        .bind(org.created)
        .bind(org.modified)
        .execute(&mut conn)
        .await
        .unwrap();
    }
}
