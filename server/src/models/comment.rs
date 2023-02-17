use super::user::GithubUserInfo;
use serde::{Deserialize, Serialize};

/// 评论
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    /// 评论 ID
    pub id: Option<i32>,
    /// 发表评论的用户信息
    /// 实现 Serialize 和 Deserialize
    pub user: Option<GithubUserInfo>,
    /// 评论内容
    pub content: String,
    /// 评论日期
    pub date: Option<chrono::NaiveDate>,
    /// 评论文章ID
    pub article: Option<i32>,
}
