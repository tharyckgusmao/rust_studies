use axum::{ response::IntoResponse, extract::{ State, Query }, Json };
use serde::Deserialize;

use crate::users_db::{ self, UsersDb };

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub async fn get_users(
    pagination: Option<Query<Pagination>>,
    State(users_db): State<UsersDb>
) -> impl IntoResponse {
    let users = users_db.read().unwrap();

    let Query(pagination) = pagination.unwrap_or_default();

    let users = users
        .values()
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(usize::MAX))
        .cloned()
        .collect::<Vec<_>>();

    Json(users)
}
