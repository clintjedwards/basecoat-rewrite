use crate::models::Colorant;
use crate::storage::Db;

impl Db {
    pub async fn list_colorants(&self, org: &str) -> Vec<Colorant> {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query_as::<_, Colorant>(
            r#"
        SELECT id, name, manufacturer, created, modified, org_id
        FROM colorants
        WHERE org_id = ?
        ORDER BY name
            "#,
        )
        .bind(org)
        .fetch_all(&mut conn)
        .await
        .unwrap()
    }

    pub async fn create_colorant(&self, colorant: &Colorant) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        INSERT INTO colorants (id, name, manufacturer, created, modified, org_id)
        VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&colorant.id)
        .bind(&colorant.name)
        .bind(&colorant.manufacturer)
        .bind(colorant.created)
        .bind(colorant.modified)
        .bind(&colorant.org_id)
        .execute(&mut conn)
        .await
        .unwrap();
    }

    pub async fn get_colorant(&self, org: &str, id: &str) -> Colorant {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query_as::<_, Colorant>(
            r#"
        SELECT id, name, manufacturer, created, modified, org_id
        FROM colorants
        WHERE org_id = ? AND id = ?
            "#,
        )
        .bind(org)
        .bind(id)
        .fetch_one(&mut conn)
        .await
        .unwrap()
    }

    pub async fn delete_colorant(&self, org: &str, id: &str) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        DELETE FROM colorants
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
