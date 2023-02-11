use ntex::web::{
    self,
    types::{Json, State},
};
use std::sync::Arc;

use crate::{errors::CustomError, models::article::Article, AppState};

#[web::get("/articles")]
pub async fn get_all_articles(
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
