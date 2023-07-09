use axum::{ extract::{ State, Path }, Json, response::IntoResponse, http::StatusCode };
use serde::{ Deserialize };
use uuid::Uuid;

use crate::users_db::{ UsersDb };

pub async fn delete_user(Path(id): Path<Uuid>, State(users_db): State<UsersDb>) -> StatusCode {
    if users_db.write().unwrap().remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
