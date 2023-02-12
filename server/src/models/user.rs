use serde::Deserialize;

/// 前端github授权登录后传上来的code
#[derive(Debug, Clone, Deserialize)]
pub struct Login {
    pub code: String,
}

/// Github 返回的 access_token
#[derive(Debug, Clone, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
}

/// Github 返回的用户信息
#[derive(Debug, Clone, Deserialize)]
pub struct GithubUserInfo {
    /// Github用户ID
    pub id: i32,
    /// 用户名(不是昵称)
    pub login: String,
    /// 用户头像地址
    pub avatar_url: String,
}
