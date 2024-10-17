use super::{AuthenticatedUser, User};
use sqlx::{postgres::PgQueryResult, types::Uuid, PgConnection};

pub async fn insert_user(
    db: &mut PgConnection,
    username: &str,
    password_hash: &str,
) -> Result<AuthenticatedUser, sqlx::Error> {
    sqlx::query_as!(
        AuthenticatedUser,
        r#"
        INSERT INTO users (username, password)
        VALUES ($1, $2)
        RETURNING
            id,
            username,
            created_at;
        "#,
        username,
        password_hash,
    )
    .fetch_one(&mut *db)
    .await
}

pub async fn get_user_by_username(
    db: &mut PgConnection,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, r"SELECT * FROM users WHERE username = $1;", username,)
        .fetch_optional(&mut *db)
        .await
}

pub async fn delete_all_user_sessions_on_reuse(
    db: &mut PgConnection,
    user_id: Uuid,
    token: &str,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM sessions
        WHERE user_id = $1 AND NOT EXISTS(
            SELECT 1
            FROM sessions
            WHERE token = $2
        );
        "#,
        user_id,
        token
    )
    .execute(&mut *db)
    .await
}

pub async fn create_session(
    db: &mut PgConnection,
    user_id: Uuid,
    token: &str,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r"INSERT INTO sessions (user_id, token) VALUES ($1, $2);",
        user_id,
        token,
    )
    .execute(&mut *db)
    .await
}

pub async fn update_session(
    db: &mut PgConnection,
    user_id: Uuid,
    old_token: &str,
    new_token: &str,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE sessions
        SET
            token = $3,
            created_at = NOW()
        WHERE user_id = $1 AND token = $2
        "#,
        user_id,
        old_token,
        new_token
    )
    .execute(&mut *db)
    .await
}

pub async fn delete_session(
    db: &mut PgConnection,
    user_id: Uuid,
    token: &str,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r"DELETE FROM sessions WHERE user_id = $1 AND token = $2;",
        user_id,
        token
    )
    .execute(&mut *db)
    .await
}
