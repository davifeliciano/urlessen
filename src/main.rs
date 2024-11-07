use rocket::{
    fairing::AdHoc,
    figment::providers::{Format, Toml},
    http::Method,
    launch, routes,
};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_db_pools::Database;
use urlessen::{
    auth::handlers::{logout, refresh, signin, signup},
    config::Config,
    db::Db,
    urls::handlers::{create_url, delete_url, get_url, get_urls_by_username, patch_url},
};

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment().merge(Toml::file("App.toml").nested());
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::custom(figment)
        .attach(AdHoc::config::<Config>())
        .attach(cors.to_cors().unwrap())
        .attach(Db::init())
        .mount("/auth", routes![signup, signin, refresh, logout])
        .mount("/urls", routes![get_url, create_url, patch_url, delete_url])
        .mount("/users", routes![get_urls_by_username])
}
