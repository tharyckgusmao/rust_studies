use axum::{ extract::{ State, Path }, Json, response::IntoResponse, http::StatusCode };
use serde::{ Serialize };
use uuid::Uuid;

use crate::{ result::AppJsonResult, db::user, state::AppState };

#[derive(Debug, Serialize)]
pub struct GetUserDto {
    name: String,
    username: String,
}

pub async fn get_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>
) -> AppJsonResult<user::Data> {
    let user = state.client
        .user()
        .find_unique(user::id::equals(id.to_string()))
        .exec().await?
        .unwrap();

    Ok(Json::from(user))
}
