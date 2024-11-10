use super::{CreateBody, PatchBody, Url};
use crate::{auth::AuthenticatedUser, db::Db, urls::repo, Validate};
use nanoid::nanoid;
use rocket::{http::Status, serde::json::Json};
use rocket_db_pools::Connection;
use sqlx::types::Uuid;

#[rocket::get("/<id>")]
pub async fn get_url(
    mut db: Connection<Db>,
    _user: AuthenticatedUser,
    id: Uuid,
) -> Result<Json<Url>, Status> {
    let url = repo::get_url(&mut db, id)
        .await
        .or(Err(Status::InternalServerError))?
        .ok_or(Status::NotFound)?;

    Ok(Json(url))
}

#[rocket::get("/<username>/urls")]
pub async fn get_urls_by_username(
    mut db: Connection<Db>,
    _user: AuthenticatedUser,
    username: &str,
) -> Result<Json<Vec<Url>>, Status> {
    let urls = repo::get_urls_by_username(&mut db, &username)
        .await
        .or(Err(Status::InternalServerError))?;

    Ok(Json(urls))
}

#[rocket::post("/", data = "<body>")]
pub async fn create_url(
    mut db: Connection<Db>,
    user: AuthenticatedUser,
    body: Json<CreateBody>,
) -> Result<Json<Url>, Status> {
    if !body.validate() {
        return Err(Status::UnprocessableEntity);
    }

    let url = repo::insert_url(
        &mut db,
        user.id,
        &body.title,
        &body.description,
        &body.long_url,
        &nanoid!(8),
    )
    .await
    .map_err(|e| match e.as_database_error() {
        Some(e) if e.is_unique_violation() => Status::Conflict, // TODO: Implement collision prevention strategy
        Some(e) if e.is_foreign_key_violation() => Status::NotFound,
        _ => Status::InternalServerError,
    })?;

    Ok(Json(url))
}

#[rocket::patch("/<id>", data = "<body>")]
pub async fn patch_url(
    mut db: Connection<Db>,
    user: AuthenticatedUser,
    id: Uuid,
    body: Json<PatchBody>,
) -> Result<Json<Url>, Status> {
    let url = repo::get_url(&mut db, id)
        .await
        .or(Err(Status::InternalServerError))?
        .ok_or(Status::NotFound)?;

    if url.creator != user.id {
        return Err(Status::Forbidden);
    }

    let url = repo::patch_url(
        &mut db,
        id,
        body.title.as_deref(),
        body.description.as_deref(),
    )
    .await
    .or(Err(Status::InternalServerError))?
    .ok_or(Status::NotFound)?;

    Ok(Json(url))
}

#[rocket::delete("/<id>")]
pub async fn delete_url(
    mut db: Connection<Db>,
    user: AuthenticatedUser,
    id: Uuid,
) -> Result<Json<Url>, Status> {
    let url = repo::get_url(&mut db, id)
        .await
        .or(Err(Status::InternalServerError))?
        .ok_or(Status::NotFound)?;

    if url.creator != user.id {
        return Err(Status::Forbidden);
    }

    let url = repo::delete_url(&mut db, id)
        .await
        .or(Err(Status::InternalServerError))?
        .ok_or(Status::NotFound)?;

    Ok(Json(url))
}
