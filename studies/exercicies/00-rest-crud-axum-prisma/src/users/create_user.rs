use axum::{ extract::State, Json };

use serde::Deserialize;

use crate::{ state::AppState, db::user, result::AppJsonResult };

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    name: String,
    username: String,
}

pub async fn create_user(
    State(clientdb): State<AppState>,
    Json(create_user_dto): Json<CreateUserDto>
) -> AppJsonResult<user::Data> {
    let user = clientdb.client
        .user()
        .create(create_user_dto.name, create_user_dto.username, vec![])
        .exec().await?;

    Ok(Json::from(user))
}
