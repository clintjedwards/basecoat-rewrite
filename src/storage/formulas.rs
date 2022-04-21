use crate::models::{Base, Colorant, Formula};
use crate::storage::Db;
use sqlx::sqlite::SqliteRow;
use sqlx::Acquire;
use sqlx::Row;

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
            bases: None,
            colorants: None,
            created: row.get("created"),
            modified: row.get("modified"),
            org_id: row.get("org_id"),
        })
        .fetch_all(&mut tx)
        .await
        .unwrap();

        for formula in &mut formulas {
            let bases = sqlx::query_as::<_, Base>(
                r#"
            SELECT bases.id, bases.name, bases.manufacturer, bases.created, bases.modified, bases.org_id
            FROM bases
            INNER JOIN formulas_bases ON bases.id=formulas_bases.base_id
            WHERE bases.org_id = ? AND formulas_bases.formula_id = ?;
                "#,
            )
            .bind(org)
            .bind(&formula.id)
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
            .bind(&formula.id)
            .fetch_all(&mut tx)
            .await
            .unwrap();

            if !bases.is_empty() {
                formula.bases = Some(bases);
            };

            if !colorants.is_empty() {
                formula.colorants = Some(colorants);
            };
        }

        tx.commit().await.unwrap();

        formulas
    }

    pub async fn create_formula(&self, formula: &Formula) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

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
        .execute(&mut conn)
        .await
        .unwrap();
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
            bases: None,
            colorants: None,
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
            formula.bases = Some(bases);
        };

        if !colorants.is_empty() {
            formula.colorants = Some(colorants);
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
}
