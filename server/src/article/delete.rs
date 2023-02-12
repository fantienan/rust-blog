use crate::{errors::CustomError, AppState};
use ntex::web::types::{Path, State};
use std::sync::Arc;

pub async fn delete_article(
    id: Path<(i32,)>,
    state: State<Arc<AppState>>,
) -> Result<String, CustomError> {
    let db_pool = &state.db_pool;

    sqlx::query!("SELECT title FROM articles WHERE id = $1", id.0)
        .fetch_one(db_pool)
        .await?;

    sqlx::query!("DELETE FROM articles WHERE id = $1", id.0)
        .execute(db_pool)
        .await?;

    Ok("删除文章成功".into())
}
