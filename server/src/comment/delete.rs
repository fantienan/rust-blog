use crate::{
    errors::CustomError,
    models::user::{Admin, User},
    AppState,
};
use ntex::web::types::{Path, State};
use std::sync::Arc;

/// 根据评论的ID删除评论
///  需要用户权限，管理员可以随意删除评论，用户只能删除自己的评论
pub async fn delete_comment(
    user: User,
    admin: Option<Admin>,
    comment_id: Path<(i32,)>,
    state: State<Arc<AppState>>,
) -> Result<String, CustomError> {
    let db_pool = &state.db_pool;
    let comment_id = comment_id.0;
    let user_id = user.id;
    let is_admin = admin.is_some();
    let rows_affected = if is_admin {
        sqlx::query!("DELETE FROM comments WHERE id = $1", comment_id)
            .execute(db_pool)
            .await?
    } else {
        sqlx::query!(
            "DELETE FROM comments WHERE id = $1 AND user_id = $2",
            comment_id,
            user_id
        )
        .execute(db_pool)
        .await?
    }
    .rows_affected();

    if rows_affected == 0 {
        Err(CustomError::NotFound(
            "删除评论失败，可能时提供的评论 ID 不正确或你没有权限删除这条评论".into(),
        ))
    } else {
        Ok("删除评论成功".into())
    }
}
