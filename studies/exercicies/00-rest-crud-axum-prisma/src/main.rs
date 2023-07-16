use std::{ net::SocketAddr, sync::Arc };
use axum::{ Router, http::StatusCode, Json, routing::{ post, get, put, delete } };
use prisma_client_rust::NewClientError;
use rest_axum_prisma::{
    users::{
        create_user::create_user,
        get_user::get_user,
        delete_user::delete_user,
        get_users::get_users,
        update_user::update_user,
    },
    state::AppState,
    db::PrismaClient,
};
use serde_json::{ Value, json };

pub async fn get_client() -> Result<PrismaClient, NewClientError> {
    PrismaClient::_builder().build().await
}

#[tokio::main]
async fn main() {
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));

    let client: Arc<PrismaClient> = Arc::new(get_client().await.ok().unwrap());
    let state = AppState { client };

    println!("Database connected...");

    let users_api = Router::new()
        .route("/", post(create_user))
        .route("/", get(get_users))
        .route("/:id", get(get_user))
        .route("/:id", delete(delete_user))
        .route("/:id", put(update_user))
        .with_state(state);

    let api = Router::new().nest("/users", users_api).fallback(api_fallback);
    let app = Router::new().nest("/api", api);
    println!("Api listen on 127.0.01:8080");
    axum::Server::bind(&address).serve(app.into_make_service()).await.unwrap();
}

async fn api_fallback() -> (StatusCode, Json<Value>) {
    let body = json!({
        "status": 404,
        "message": "Not Found"
    });

    (StatusCode::NOT_FOUND, Json(body))
}
