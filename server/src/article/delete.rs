use crate::{errors::CustomError, models::user::Admin, AppState};
use ntex::web::types::{Path, State};
use std::sync::Arc;

pub async fn delete_article(
    _: Admin,
    id: Path<(i32,)>,
    state: State<Arc<AppState>>,
) -> Result<String, CustomError> {
    let db_pool = &state.db_pool;

    if sqlx::query!("SELECT title FROM articles WHERE id = $1", id.0)
        .fetch_optional(db_pool)
        .await?
        .is_none()
    {
        return Err(CustomError::BadRequest("无法对不错在的记录进行删除".into()));
    }

    sqlx::query!("DELETE FROM articles WHERE id = $1", id.0)
        .execute(db_pool)
        .await?;

    Ok("删除文章成功".into())
}
