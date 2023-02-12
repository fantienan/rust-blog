use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
// 文章的详情
pub struct Article {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ArticlePreview {
    pub id: i32,
    pub title: String,
    pub date: Option<chrono::NaiveDate>,
}
