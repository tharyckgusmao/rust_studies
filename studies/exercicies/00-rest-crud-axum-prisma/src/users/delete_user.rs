use axum::{ extract::{ State, Path }, Json };
use uuid::Uuid;

use crate::{ state::AppState, result::AppJsonResult, db::user };

pub async fn delete_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>
) -> AppJsonResult<user::Data> {
    let user = state.client.user().delete(user::id::equals(id.to_string())).exec().await?;

    Ok(Json::from(user))
}
