use anyhow::Result;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;
use shared::{User, UserRole};

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        pool: &SqlitePool,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, role, created_at)
            VALUES (?, ?, ?, ?, 'user', ?)
            "#,
        )
        .bind(id.to_string())
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(User {
            id,
            username: username.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            role: UserRole::User,
            created_at: now,
        })
    }

    pub async fn find_by_email(pool: &SqlitePool, email: &str) -> Result<Option<User>> {
        let row: Option<(String, String, String, String, String, String)> = sqlx::query_as(
            "SELECT id, username, email, password_hash, role, created_at FROM users WHERE email = ?",
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        match row {
            Some((id, username, email, password_hash, role, created_at)) => {
                let role = match role.as_str() {
                    "admin" => UserRole::Admin,
                    _ => UserRole::User,
                };
                Ok(Some(User {
                    id: id.parse()?,
                    username,
                    email,
                    password_hash,
                    role,
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn find_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<User>> {
        let row: Option<(String, String, String, String, String, String)> = sqlx::query_as(
            "SELECT id, username, email, password_hash, role, created_at FROM users WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;

        match row {
            Some((id, username, email, password_hash, role, created_at)) => {
                let role = match role.as_str() {
                    "admin" => UserRole::Admin,
                    _ => UserRole::User,
                };
                Ok(Some(User {
                    id: id.parse()?,
                    username,
                    email,
                    password_hash,
                    role,
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn email_exists(pool: &SqlitePool, email: &str) -> Result<bool> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE email = ?")
            .bind(email)
            .fetch_one(pool)
            .await?;
        Ok(count.0 > 0)
    }

    pub async fn username_exists(pool: &SqlitePool, username: &str) -> Result<bool> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(pool)
            .await?;
        Ok(count.0 > 0)
    }
}
