use crate::{errors::CustomError, AppState};
use cookie::Cookie;
use ntex::{
    http::{HttpMessage, Payload},
    web::{ErrorRenderer, FromRequest, HttpRequest},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{future::Future, pin::Pin, sync::Arc};

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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GithubUserInfo {
    /// Github用户ID
    pub id: i32,
    /// 用户名(不是昵称)
    pub login: String,
    /// 用户头像地址
    pub avatar_url: String,
}

/// 网站所有用户，包括管理员
#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
}

/// 网站管理员用于身份认证
#[derive(Debug, Clone)]
pub struct Admin {
    pub id: i32,
}

// 实现FromRequest trait
// 可以从请求中提前用户数据并验证用户信息
// async fn handler(user: User / admin: Admin)
// 这样就可以为具体的handler添加身份认证了
// 参考 Json<T>
impl<E: ErrorRenderer> FromRequest<E> for User {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // 注意：下面两个变量类型不能出现引用（req），否则会出现声明周期问题（future）
        let db_pool = Arc::clone(req.app_state::<Arc<AppState>>().unwrap())
            .db_pool
            .clone();

        // Cookies 中的access token
        let access_token = req.cookie("ACCESS_TOKEN");
        let fut = async move {
            let access_token = match access_token {
                Some(c) => c,
                None => return Err(CustomError::AuthFailed("你还没有登录".into())),
            };

            let user_id = match get_user_id(&access_token).await {
                Ok(id) => id,
                Err(e) => return Err(e),
            };

            if sqlx::query!("SELECT id FROM users WHERE id = $1", user_id)
                .fetch_optional(&db_pool)
                .await
                .unwrap()
                .is_none()
            {
                // 用户没有在本站使用Github登录过
                return Err(CustomError::AuthFailed(
                    "你还没有在本站使用Github登录过，请登录".into(),
                ));
            };

            Ok(Self { id: user_id })
        };

        Box::pin(fut)
    }
}

impl<E: ErrorRenderer> FromRequest<E> for Admin {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // 注意：下面两个变量类型不能出现引用（req），否则会出现声明周期问题（future）
        let db_pool = Arc::clone(req.app_state::<Arc<AppState>>().unwrap())
            .db_pool
            .clone();

        // Cookies 中的access token
        let access_token = req.cookie("ACCESS_TOKEN");
        println!("{:?}", access_token);
        let fut = async move {
            let access_token = match access_token {
                Some(c) => c,
                None => return Err(CustomError::AuthFailed("你还没有登录".into())),
            };

            let user_id = match get_user_id(&access_token).await {
                Ok(id) => id,
                Err(e) => return Err(e),
            };
            if sqlx::query!("SELECT id FROM users WHERE id = $1", user_id)
                .fetch_optional(&db_pool)
                .await?
                .is_some()
            {
                if user_id != 23467070 {
                    return Err(CustomError::AuthFailed(
                        "你不是管理员，无权执行该操作".into(),
                    ));
                }
            } else {
                return Err(CustomError::AuthFailed(
                    "你还没有在本站使用Github登录过，请登录".into(),
                ));
            }

            Ok(Self { id: user_id })
        };

        Box::pin(fut)
    }
}

async fn get_user_id(access_token: &Cookie<'_>) -> Result<i32, CustomError> {
    let client = Client::new();

    let user_info = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token.value())
        // github 的 API 要求设置 UA
        .header("User-Agent", "fantienan")
        .send()
        .await;

    let user_id = match user_info {
        Ok(r) => match r.json::<GithubUserInfo>().await {
            Ok(i) => i.id,
            Err(_) => {
                // 无法解析，可能是Github返回了错误消息
                return Err(CustomError::BadRequest(
                    "无法获取Github用户信息，可能是access token不正确，请重新登录".into(),
                ));
            }
        },
        Err(_) => {
            // 请求错误
            return Err(CustomError::InternalServerError(
                "无法获取Github用户信息，请重试".into(),
            ));
        }
    };
    Ok(user_id)
}
