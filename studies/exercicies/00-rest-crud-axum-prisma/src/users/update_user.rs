use std::vec;

use axum::{ extract::{ State, Path }, Json, response::IntoResponse, http::StatusCode };
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use crate::{ result::AppJsonResult, db::user, state::AppState };

#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    name: Option<String>,
    username: Option<String>,
}

pub async fn update_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(update_user_dto): Json<UpdateUserDto>
) -> AppJsonResult<user::Data> {
    let user = state.client
        .user()
        .update(
            user::id::equals(id.to_string()),
            vec![
                user::name::set(update_user_dto.name.unwrap()),
                user::username::set(update_user_dto.username.unwrap())
            ]
        )
        .exec().await?;

    Ok(Json::from(user))
}
