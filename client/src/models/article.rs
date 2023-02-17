use serde::{Deserialize, Serialize};

/// 文章的详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub date: Option<String>
}

/// 文章预览
#[derive(Debug, Clone,Deserialize)]
pub struct ArticlePreview {
    pub id: i32,
    pub title: String,
    pub date: String
}