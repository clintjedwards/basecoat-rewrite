use crate::models::{Base, Colorant, Formula};
use crate::storage::Db;
use sqlx::sqlite::SqliteRow;
use sqlx::{Acquire, Row};

impl Db {
    pub async fn list_formulas(&self, org: &str) -> Vec<Formula> {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        let mut formulas = sqlx::query(
            r#"
        SELECT id, name, number, notes, created, modified, org_id
        FROM formulas
        WHERE org_id = ?
            "#,
        )
        .bind(org)
        .map(|row: SqliteRow| Formula {
            id: row.get("id"),
            name: row.get("name"),
            number: row.get("number"),
            notes: row.get("notes"),
            bases: vec![],
            colorants: vec![],
            created: row.get("created"),
            modified: row.get("modified"),
            org_id: row.get("org_id"),
        })
        .fetch_all(&mut tx)
        .await
        .unwrap();

        for formula in &mut formulas {
            let bases = sqlx::query(
                r#"
            SELECT bases.id, bases.name, bases.manufacturer, bases.created, bases.modified, formulas_bases.amount, bases.org_id
            FROM bases
            INNER JOIN formulas_bases ON bases.id=formulas_bases.base_id
            WHERE bases.org_id = ? AND formulas_bases.formula_id = ?;
                "#,
            )
            .bind(org)
            .bind(&formula.id)
            .map(|row: SqliteRow| Base {
                id: row.get("bases.id"),
                name: row.get("bases.name"),
                manufacturer: row.get("bases.manufacturer"),
                amount: row.get("formulas_bases.amount"),
                created: row.get("bases.created"),
                modified: row.get("bases.modified"),
                org_id: row.get("bases.org_id"),
            })
            .fetch_all(&mut tx)
            .await
            .unwrap();

            let colorants = sqlx::query(
                r#"
            SELECT colorants.id, colorants.name, colorants.manufacturer, colorants.created, colorants.modified, formulas_colorants.amount, colorants.org_id
            FROM colorants
            INNER JOIN formulas_colorants ON colorants.id=formulas_colorants.colorant_id
            WHERE colorants.org_id = ? AND formulas_colorants.formula_id = ?;
                "#,
            )
            .bind(org)
            .bind(&formula.id)
            .map(|row: SqliteRow| Colorant {
                id: row.get("colorants.id"),
                name: row.get("colorants.name"),
                manufacturer: row.get("colorants.manufacturer"),
                amount: row.get("formulas_colorants.amount"),
                created: row.get("colorants.created"),
                modified: row.get("colorants.modified"),
                org_id: row.get("colorants.org_id"),
            })
            .fetch_all(&mut tx)
            .await
            .unwrap();

            if !bases.is_empty() {
                formula.bases = bases;
            };

            if !colorants.is_empty() {
                formula.colorants = colorants;
            };
        }

        tx.commit().await.unwrap();

        formulas
    }

    pub async fn create_formula(&self, formula: &Formula) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        sqlx::query(
            r#"
        INSERT INTO formulas (id, name, number, notes, created, modified, org_id)
        VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&formula.id)
        .bind(&formula.name)
        .bind(&formula.number)
        .bind(&formula.notes)
        .bind(formula.created)
        .bind(formula.modified)
        .bind(&formula.org_id)
        .execute(&mut tx)
        .await
        .unwrap();

        if !formula.bases.is_empty() {
            for base in &formula.bases {
                sqlx::query(
                    r#"
        INSERT INTO formulas_bases (formula_id, base_id, org_id, amount)
        VALUES (?, ?, ?, ?)
            "#,
                )
                .bind(&formula.id)
                .bind(&base.id)
                .bind(&formula.org_id)
                .bind(&base.amount)
                .execute(&mut tx)
                .await
                .unwrap();
            }
        }

        if !formula.colorants.is_empty() {
            for colorant in &formula.colorants {
                sqlx::query(
                    r#"
        INSERT INTO formulas_colorants (formula_id, colorant_id, org_id, amount)
        VALUES (?, ?, ?, ?)
            "#,
                )
                .bind(&formula.id)
                .bind(&colorant.id)
                .bind(&formula.org_id)
                .bind(&colorant.amount)
                .execute(&mut tx)
                .await
                .unwrap();
            }
        }

        tx.commit().await.unwrap();
    }

    pub async fn update_formula(&self, formula: &Formula) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        sqlx::query(
            r#"
        UPDATE formulas
        SET name = ?, number = ?, notes = ?, modified = ?,
        WHERE org_id = ? AND id = ?
            "#,
        )
        .bind(&formula.name)
        .bind(&formula.number)
        .bind(&formula.notes)
        .bind(formula.modified)
        .bind(&formula.org_id)
        .bind(&formula.id)
        .execute(&mut tx)
        .await
        .unwrap();

        // Erase all connected formula bases so we can reattach them
        sqlx::query(
            r#"
        DELETE FROM formulas_bases
        WHERE org_id = ? AND formula_id = ?
            "#,
        )
        .bind(&formula.org_id)
        .bind(&formula.id)
        .execute(&mut tx)
        .await
        .unwrap();

        // Erase all connected formula colorants so we can reattach them
        sqlx::query(
            r#"
        DELETE FROM formulas_colorants
        WHERE org_id = ? AND formula_id = ?
            "#,
        )
        .bind(&formula.org_id)
        .bind(&formula.id)
        .execute(&mut tx)
        .await
        .unwrap();

        for base in &formula.bases {
            sqlx::query(
                r#"
            INSERT INTO formulas_bases (formula_id, base_id, org_id, amount)
            VALUES (?, ?, ?, ?)
            "#,
            )
            .bind(&formula.id)
            .bind(&base.id)
            .bind(&formula.org_id)
            .bind(&base.amount)
            .execute(&mut tx)
            .await
            .unwrap();
        }

        for colorant in &formula.colorants {
            sqlx::query(
                r#"
            INSERT INTO formulas_colorants (formula_id, colorant_id, org_id, amount)
            VALUES (?, ?, ?, ?)
            "#,
            )
            .bind(&formula.id)
            .bind(&colorant.id)
            .bind(&formula.org_id)
            .bind(&colorant.amount)
            .execute(&mut tx)
            .await
            .unwrap();
        }

        tx.commit().await.unwrap();
    }

    pub async fn get_formula(&self, org: &str, id: &str) -> Formula {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        let mut formula = sqlx::query(
            r#"
        SELECT id, name, number, notes, created, modified, org_id
        FROM formulas
        WHERE org_id = ? AND id = ?
            "#,
        )
        .bind(org)
        .bind(id)
        .map(|row: SqliteRow| Formula {
            id: row.get("id"),
            name: row.get("name"),
            number: row.get("number"),
            notes: row.get("notes"),
            bases: vec![],
            colorants: vec![],
            created: row.get("created"),
            modified: row.get("modified"),
            org_id: row.get("org_id"),
        })
        .fetch_one(&mut tx)
        .await
        .unwrap();

        let bases = sqlx::query_as::<_, Base>(
                r#"
            SELECT bases.id, bases.name, bases.manufacturer, bases.created, bases.modified, bases.org_id
            FROM bases
            INNER JOIN formulas_bases ON bases.id=formulas_bases.base_id
            WHERE bases.org_id = ? AND formulas_bases.formula_id = ?;
                "#,
            )
            .bind(org)
            .bind(id)
            .fetch_all(&mut tx)
            .await
            .unwrap();

        let colorants = sqlx::query_as::<_, Colorant>(
                r#"
            SELECT colorants.id, colorants.name, colorants.manufacturer, colorants.created, colorants.modified, colorants.org_id
            FROM colorants
            INNER JOIN formulas_colorants ON colorants.id=formulas_colorants.colorant_id
            WHERE colorants.org_id = ? AND formulas_colorants.formula_id = ?;
                "#,
            )
            .bind(org)
            .bind(id)
            .fetch_all(&mut tx)
            .await
            .unwrap();

        if !bases.is_empty() {
            formula.bases = bases;
        };

        if !colorants.is_empty() {
            formula.colorants = colorants;
        };

        tx.commit().await.unwrap();

        formula
    }

    pub async fn delete_formula(&self, org: &str, id: &str) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        DELETE FROM formulas
        WHERE org_id = ? AND id = ?
            "#,
        )
        .bind(org)
        .bind(id)
        .execute(&mut conn)
        .await
        .unwrap();
    }

    pub async fn attach_formula_to_job(&self, org_id: &str, formula_id: &str, job_id: &str) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        INSERT INTO jobs_formulas (job_id, formula_id, org_id)
        VALUES (?, ?, ?)
            "#,
        )
        .bind(job_id)
        .bind(formula_id)
        .bind(org_id)
        .execute(&mut conn)
        .await
        .unwrap();
    }

    pub async fn detach_formula_from_job(&self, org_id: &str, formula_id: &str, job_id: &str) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        DELETE FROM jobs_formulas
        WHERE job_id = ? AND formula_id = ? AND org_id = ?
            "#,
        )
        .bind(job_id)
        .bind(formula_id)
        .bind(org_id)
        .execute(&mut conn)
        .await
        .unwrap();
    }
}
