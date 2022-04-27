use crate::models::Contractor;
use crate::storage::Db;

impl Db {
    pub async fn list_contractors(&self, org: &str) -> Vec<Contractor> {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query_as::<_, Contractor>(
            r#"
        SELECT id, name, contact, created, modified, org_id
        FROM contractors
        WHERE org_id = ?
            "#,
        )
        .bind(org)
        .fetch_all(&mut conn)
        .await
        .unwrap()
    }

    pub async fn create_contractor(&self, contractor: &Contractor) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        INSERT INTO contractors (id, name, contact, created, modified, org_id)
        VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&contractor.id)
        .bind(&contractor.name)
        .bind(&contractor.contact)
        .bind(contractor.created)
        .bind(contractor.modified)
        .bind(&contractor.org_id)
        .execute(&mut conn)
        .await
        .unwrap();
    }

    pub async fn update_contractor(&self, contractor: &Contractor) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        UPDATE contractors
        SET name = ?, contact = ?, modified = ?,
        WHERE org_id = ? AND id = ?
            "#,
        )
        .bind(&contractor.name)
        .bind(&contractor.contact)
        .bind(contractor.modified)
        .bind(&contractor.org_id)
        .bind(&contractor.id)
        .execute(&mut conn)
        .await
        .unwrap();
    }

    pub async fn get_contractor(&self, org: &str, id: &str) -> Contractor {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query_as::<_, Contractor>(
            r#"
        SELECT id, name, contact, created, modified, org_id
        FROM contractors
        WHERE org_id = ? AND id = ?
            "#,
        )
        .bind(org)
        .bind(id)
        .fetch_one(&mut conn)
        .await
        .unwrap()
    }

    pub async fn delete_contractor(&self, org: &str, id: &str) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        DELETE FROM contractors
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
