use std::sync::Arc;

use crate::db::PrismaClient;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<PrismaClient>,
}
