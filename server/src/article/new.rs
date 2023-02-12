use ntex::web::{
    types::{Json, State},
    HttpResponse, Responder,
};
use std::sync::Arc;

use crate::{errors::CustomError, models::article::Article, AppState};

pub async fn new_article(
    article: Json<Article>,
    state: State<Arc<AppState>>,
) -> Result<impl Responder, CustomError> {
    let db_pool = &state.db_pool;

    sqlx::query!(
        "INSERT INTO articles (title, content) VALUES ($1, $2)",
        article.title,
        article.content
    )
    .execute(db_pool)
    .await?;

    Ok(HttpResponse::Created().body("新增文章成功"))
}
