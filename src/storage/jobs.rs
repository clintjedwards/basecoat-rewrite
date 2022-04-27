use crate::models::Job;
use crate::storage::Db;

impl Db {
    pub async fn list_jobs(&self, org: &str, contractor_id: &str) -> Vec<Job> {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query_as::<_, Job>(
            r#"
        SELECT id, name, address, contact, notes, created, modified, contractor_id, org_id
        FROM jobs
        WHERE org_id = ? AND contractor_id = ?
            "#,
        )
        .bind(org)
        .bind(contractor_id)
        .fetch_all(&mut conn)
        .await
        .unwrap()
    }

    pub async fn create_job(&self, job: &Job) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        INSERT INTO jobs (id, name, address, contact, notes, created, modified, contractor_id, org_id)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&job.id)
        .bind(&job.name)
        .bind(&job.address)
        .bind(&job.contact)
        .bind(&job.notes)
        .bind(job.created)
        .bind(job.modified)
        .bind(&job.contractor_id)
        .bind(&job.org_id)
        .execute(&mut conn)
        .await
        .unwrap();
    }

    pub async fn update_job(&self, job: &Job) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        UPDATE jobs
        SET name = ?, address = ?, contact = ?, notes = ?, modified = ?, contractor_id = ?
        WHERE org_id = ? AND id = ? AND contractor_id = ?
            "#,
        )
        .bind(&job.name)
        .bind(&job.address)
        .bind(&job.contact)
        .bind(&job.notes)
        .bind(job.modified)
        .bind(&job.contractor_id)
        .bind(&job.org_id)
        .bind(&job.id)
        .bind(&job.contractor_id)
        .execute(&mut conn)
        .await
        .unwrap();
    }

    pub async fn get_job(&self, org: &str, id: &str, contractor_id: &str) -> Job {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query_as::<_, Job>(
            r#"
        SELECT id, name, address, contact, notes, created, modified, contractor_id, org_id
        FROM jobs
        WHERE org_id = ? AND id = ? AND contractor_id = ?
            "#,
        )
        .bind(org)
        .bind(id)
        .bind(contractor_id)
        .fetch_one(&mut conn)
        .await
        .unwrap()
    }

    pub async fn delete_job(&self, org: &str, id: &str, contractor_id: &str) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        DELETE FROM jobs
        WHERE org_id = ? AND id = ? AND contractor_id = ?
            "#,
        )
        .bind(org)
        .bind(id)
        .bind(contractor_id)
        .execute(&mut conn)
        .await
        .unwrap();
    }
}
