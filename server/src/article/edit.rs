use crate::{errors::CustomError, models::article::Article, AppState};
use ntex::web::types::{Json, State};
use std::sync::Arc;

pub async fn edit_article(
    article: Json<Article>,
    state: State<Arc<AppState>>,
) -> Result<String, CustomError> {
    let db_pool = &state.db_pool;

    let id = match article.id {
        Some(id) => id,
        None => return Err(CustomError::BadRequest("请提供要修改的文章ID".into())),
    };

    sqlx::query!("SELECT title FROM articles WHERE id = $1", id)
        .fetch_one(db_pool)
        .await?;

    sqlx::query!(
        "UPDATE articles SET title = $1, content = $2 WHERE id = $3",
        article.title,
        article.content,
        id
    )
    .execute(db_pool)
    .await?;

    Ok("修改文章成功!".into())
}
