use sqlx::PgPool;
use uuid::Uuid;

use crate::db::models::{ApiToken, User};

pub async fn upsert_user(
    pool: &PgPool,
    email: &str,
    name: Option<&str>,
    avatar_url: Option<&str>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, name, avatar_url)
        VALUES ($1, $2, $3)
        ON CONFLICT (email) DO UPDATE SET
            name = COALESCE(EXCLUDED.name, users.name),
            avatar_url = COALESCE(EXCLUDED.avatar_url, users.avatar_url),
            updated_at = NOW()
        RETURNING *
        "#,
    )
    .bind(email)
    .bind(name)
    .bind(avatar_url)
    .fetch_one(pool)
    .await
}

pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

#[allow(dead_code)]
pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
}

pub async fn create_api_token(
    pool: &PgPool,
    user_id: Uuid,
    name: &str,
    token_hash: &str,
) -> Result<ApiToken, sqlx::Error> {
    sqlx::query_as::<_, ApiToken>(
        r#"
        INSERT INTO api_tokens (user_id, name, token_hash)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(name)
    .bind(token_hash)
    .fetch_one(pool)
    .await
}

pub async fn get_api_token_by_hash(
    pool: &PgPool,
    token_hash: &str,
) -> Result<Option<ApiToken>, sqlx::Error> {
    sqlx::query_as::<_, ApiToken>(
        "SELECT * FROM api_tokens WHERE token_hash = $1 AND (expires_at IS NULL OR expires_at > NOW())",
    )
    .bind(token_hash)
    .fetch_optional(pool)
    .await
}

pub async fn get_user_api_tokens(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<ApiToken>, sqlx::Error> {
    sqlx::query_as::<_, ApiToken>(
        "SELECT * FROM api_tokens WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn delete_api_token(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM api_tokens WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn update_api_token_last_used(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE api_tokens SET last_used_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
