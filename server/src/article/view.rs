use ntex::web::types::{Json, Path, State};
use std::sync::Arc;

use crate::{errors::CustomError, models::article::Article, AppState};

/// 预览文章
pub async fn get_articles_preview(
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<Article>>, CustomError> {
    let db_pool = &state.db_pool;

    let articles = sqlx::query!(r#"SELECT * FROM articles"#)
        .fetch_all(db_pool)
        .await?
        .iter()
        .map(|i| Article {
            id: Some(i.id),
            title: i.title.clone(),
            content: i.content.clone(),
            date: i.date,
        })
        .collect();

    Ok(Json(articles))
}

/// 通过ID获取单篇文章
pub async fn get_article(
    id: Path<(i32,)>,
    state: State<Arc<AppState>>,
) -> Result<Json<Article>, CustomError> {
    let db_pool = &state.db_pool;

    let article = sqlx::query!(
        "SELECT title, content, date FROM articles WHERE id = $1",
        id.0
    )
    .fetch_one(db_pool)
    .await?;

    let article = Article {
        id: None,
        title: article.title.clone(),
        content: article.content.clone(),
        date: article.date,
    };

    Ok(Json(article))
}
