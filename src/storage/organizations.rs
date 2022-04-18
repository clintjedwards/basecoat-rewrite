use crate::models::Organization;
use crate::storage::Db;

impl Db {
    pub async fn list_organizations(&self) -> Vec<Organization> {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query_as::<_, Organization>(
            r#"
        SELECT id, name, created, modified
        FROM organizations
        ORDER BY name
            "#,
        )
        .fetch_all(&mut conn)
        .await
        .unwrap()
    }

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

    pub async fn get_organization(&self, id: &str) -> Organization {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query_as::<_, Organization>(
            r#"
        SELECT id, name, created, modified
        FROM organizations
        WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&mut conn)
        .await
        .unwrap()
    }
}
