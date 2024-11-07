use super::Url;
use sqlx::{types::Uuid, PgConnection};

pub async fn get_url(db: &mut PgConnection, id: Uuid) -> Result<Option<Url>, sqlx::Error> {
    sqlx::query_as!(Url, "SELECT * FROM urls WHERE id = $1;", id,)
        .fetch_optional(&mut *db)
        .await
}

pub async fn get_urls_by_username(
    db: &mut PgConnection,
    username: &str,
) -> Result<Vec<Url>, sqlx::Error> {
    sqlx::query_as!(
        Url,
        r#"
        SELECT urls.*
        FROM urls
        JOIN users ON users.id = urls.creator
        WHERE users.username = $1;
        "#,
        username,
    )
    .fetch_all(&mut *db)
    .await
}

pub async fn insert_url(
    db: &mut PgConnection,
    creator: Uuid,
    title: &str,
    description: Option<&str>,
    long_url: &str,
    short_url: &str,
) -> Result<Url, sqlx::Error> {
    sqlx::query_as!(
        Url,
        r#"
        INSERT INTO urls (
            creator,
            title,
            description,
            long_url,
            short_url
        )
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *;
        "#,
        creator,
        title,
        description,
        long_url,
        short_url
    )
    .fetch_one(&mut *db)
    .await
}

pub async fn patch_url(
    db: &mut PgConnection,
    id: Uuid,
    title: Option<&str>,
    description: Option<&str>,
) -> Result<Option<Url>, sqlx::Error> {
    match (title, description) {
        (None, None) => get_url(db, id).await,
        (None, Some(description)) => {
            sqlx::query_as!(
                Url,
                r#"
                UPDATE urls SET
                    description = $2,
                    updated_at = NOW()
                WHERE id = $1
                RETURNING *;
                "#,
                id,
                description
            )
            .fetch_optional(&mut *db)
            .await
        }
        (Some(title), None) => {
            sqlx::query_as!(
                Url,
                r#"
                UPDATE urls SET
                    title = $2,
                    updated_at = NOW()
                WHERE id = $1
                RETURNING *;
                "#,
                id,
                title
            )
            .fetch_optional(&mut *db)
            .await
        }
        (Some(title), Some(description)) => {
            sqlx::query_as!(
                Url,
                r#"
                UPDATE urls SET
                    title = $2,
                    description = $3,
                    updated_at = NOW()
                WHERE id = $1
                RETURNING *;
                "#,
                id,
                title,
                description
            )
            .fetch_optional(&mut *db)
            .await
        }
    }
}

pub async fn delete_url(db: &mut PgConnection, id: Uuid) -> Result<Option<Url>, sqlx::Error> {
    sqlx::query_as!(Url, "DELETE FROM urls WHERE id = $1 RETURNING *;", id,)
        .fetch_optional(&mut *db)
        .await
}
