use std::{ net::SocketAddr };

use axum::{ Router, http::StatusCode, Json, routing::{ post, get, put, delete } };
use rest_axum::{
    users_db::UsersDb,
    users::{
        create_user::create_user,
        get_user::get_user,
        update_user::update_user,
        delete_user::delete_user,
        get_users::get_users,
    },
};
use serde_json::{ Value, json };

#[tokio::main]
async fn main() {
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));

    let users_db = UsersDb::default();

    let users_api = Router::new()
        .route("/", get(get_users))
        .route("/", post(create_user))
        .route("/:id", get(get_user))
        .route("/:id", put(update_user))
        .route("/:id", delete(delete_user))
        .with_state(users_db);

    let api = Router::new().nest("/users", users_api).fallback(api_fallback);
    let app = Router::new().nest("/api", api);

    axum::Server::bind(&address).serve(app.into_make_service()).await.unwrap();
}

async fn api_fallback() -> (StatusCode, Json<Value>) {
    let body = json!({
        "status": 404,
        "message": "Not Found"
    });

    (StatusCode::NOT_FOUND, Json(body))
}
