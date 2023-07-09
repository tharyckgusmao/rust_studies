use axum::{ extract::{ State, Path }, Json, response::IntoResponse, http::StatusCode };
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use crate::users_db::{ UsersDb };

#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    name: Option<String>,
    username: Option<String>,
}

pub async fn update_user(
    Path(id): Path<Uuid>,
    State(users_db): State<UsersDb>,
    Json(update_user_dto): Json<UpdateUserDto>
) -> Result<impl IntoResponse, StatusCode> {
    let mut user = users_db.read().unwrap().get(&id).cloned().ok_or(StatusCode::NOT_FOUND)?;

    if let Some(name) = update_user_dto.name {
        user.name = name;
    }
    if let Some(username) = update_user_dto.username {
        user.username = username;
    }

    users_db.write().unwrap().insert(user.id, user.clone());

    Ok((StatusCode::NO_CONTENT, Json(user)))
}
