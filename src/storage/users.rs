use sqlx::Acquire;

use crate::models::{User, UserState};
use crate::storage::Db;

impl Db {
    pub async fn list_users(&self, org: &str) -> Vec<User> {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query_as::<_, User>(
            r#"
        SELECT name, state, created, modified, org_id
        FROM users
        WHERE org_id = ?
        ORDER BY name
            "#,
        )
        .bind(org)
        .fetch_all(&mut conn)
        .await
        .unwrap()
    }

    pub async fn create_user(&self, user: &User) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        INSERT INTO users (name, hash, state, created, modified, org_id)
        VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&user.name)
        .bind(&user.hash)
        .bind(&user.state)
        .bind(user.created)
        .bind(user.modified)
        .bind(&user.org_id)
        .execute(&mut conn)
        .await
        .unwrap();
    }

    pub async fn get_user(&self, org: &str, name: &str) -> User {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query_as::<_, User>(
            r#"
        SELECT name, hash, state, created, modified, org_id
        FROM users
        WHERE org_id = ? AND name = ?
            "#,
        )
        .bind(org)
        .bind(name)
        .fetch_one(&mut conn)
        .await
        .unwrap()
    }

    pub async fn reset_user_password(&self, org: &str, name: &str, hash: &str) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query(
            r#"
        UPDATE users
        SET hash = ?
        WHERE org_id = ? AND name = ?
            "#,
        )
        .bind(hash)
        .bind(org)
        .bind(name)
        .execute(&mut conn)
        .await
        .unwrap();
    }

    pub async fn toggle_user_state(&self, org: &str, name: &str) {
        let mut conn = self.conn.as_ref().unwrap().acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        let current_user = sqlx::query_as::<_, User>(
            r#"
        SELECT name, hash, state, created, modified, org_id
        FROM users
        WHERE org_id = ? AND name = ?
            "#,
        )
        .bind(org)
        .bind(name)
        .fetch_one(&mut tx)
        .await
        .unwrap();

        let opposite_state = match current_user.state {
            UserState::Active => UserState::Disabled,
            UserState::Disabled => UserState::Active,
            UserState::Unknown => {
                panic!("user in incorrect state")
            }
        };

        sqlx::query(
            r#"
        UPDATE users
        SET state = ?
        WHERE org_id = ? AND name = ?
            "#,
        )
        .bind(opposite_state)
        .bind(org)
        .bind(name)
        .execute(&mut tx)
        .await
        .unwrap();

        tx.commit().await.unwrap();
    }
}
