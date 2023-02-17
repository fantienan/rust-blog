use crate::{
    errors::CustomError,
    models::{comment::Comment, user::GithubUserInfo},
    AppState,
};
use ntex::web::types::{Json, Path, State};
use std::sync::Arc;

/// 通过文章ID获取该文章的所有评论，包含发表评论的用户信息
pub async fn get_comments_for_article(
    article_id: Path<(i32,)>,
    state: State<Arc<AppState>>,
) -> Result<Json<Vec<Comment>>, CustomError> {
    let db_pool = &state.db_pool;
    let article_id = article_id.0;
    // 查找对应文章的所有评论，拿到当前用户的用户信息以及评论信息
    let comments = sqlx::query!(
        "SELECT comments.user_id, comments.content, comments.date, users.name, users.avatar_url FROM comments JOIN users ON comments.user_id = users.id WHERE comments.article = $1",
        article_id
    ).fetch_all(db_pool).await?.iter().map(|i| Comment {
        id: None,
        user: Some(GithubUserInfo {
            id: i.user_id,
            login: i.name.clone(),
            avatar_url: i.avatar_url.clone(),
        }),
        content: i.content.clone(),
        date:Some(i.date),
        article: None
    }).collect();

    Ok(Json(comments))
}
