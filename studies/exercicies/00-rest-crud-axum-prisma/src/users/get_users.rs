use axum::{ response::IntoResponse, extract::{ State, Query }, Json };
use serde::Deserialize;

use crate::{ state::AppState, db::user, result::AppJsonResult };

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

pub async fn get_users(
    pagination: Option<Query<Pagination>>,
    State(state): State<AppState>
) -> AppJsonResult<Vec<user::Data>> {
    let Query(pagination) = pagination.unwrap_or_default();

    let users = state.client
        .user()
        .find_many(vec![])
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(i64::MAX))
        .exec().await?;

    Ok(Json::from(users))
}
