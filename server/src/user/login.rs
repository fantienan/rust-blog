use crate::{
    errors::CustomError,
    models::user::{AccessToken, GithubUserInfo, Login},
    AppState,
};
use cookie::{time::Duration, Cookie};
use ntex::{
    http::Response,
    web::{
        types::{Json, State},
        Responder,
    },
};
use reqwest::Client;
use std::sync::Arc;

const CLIENT_ID: &str = "f7c63bbca884d820cc24";
const CLIENT_SECRET: &str = "7f3bc5172672708274c166e2bc30fa142a1bee04";

/// 接收传过来的code，获取access_token，得到用户数据并入库
pub async fn github_login(
    code: Json<Login>,
    state: State<Arc<AppState>>,
) -> Result<impl Responder, CustomError> {
    let code = &code.code;

    // HTTP Client
    let client = Client::new();

    // 获取access_token
    // 把Accept设置为json，让github的API给我们返回JSON格式的数据
    let access_token = client.post(format!(
        "https://github.com/login/oauth/access_token?client_id={CLIENT_ID}&client_secret={CLIENT_SECRET}&code={code}"
    ))
    .header("Accept", "application/json")
    .send()
    .await;

    let access_token = match access_token {
        Ok(r) => match r.json::<AccessToken>().await {
            Ok(r) => r.access_token,
            Err(_) => {
                return Err(CustomError::AuthFailed(
                    "code 无效（可能已经过期），请重新使用Github登录".into(),
                ));
            }
        },
        Err(_) => {
            return Err(CustomError::InternalServerError(
                "无法获取access_token，请重试".into(),
            ));
        }
    };

    let user_info = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token.clone())
        // github 的 API 要求设置 UA
        .header("User-Agent", "fantienan")
        .send()
        .await;

    let user_info = match user_info {
        Ok(r) => r.json::<GithubUserInfo>().await.unwrap(),
        Err(_) => {
            return Err(CustomError::InternalServerError(
                "无法获取Github用户信息， 请重试".into(),
            ))
        }
    };
    // 设置cookie，这样用户就不需要重复登录了
    let mut cookie = Cookie::new("ACCESS_TOKEN", access_token);
    cookie.set_path("/");
    cookie.set_max_age(Duration::days(7));
    cookie.set_http_only(true);

    // 把用户信息入库
    let db_pool = &state.db_pool;

    // 如果有相同ID的记录就更更新，反之就新增
    sqlx::query!(
        "INSERT INTO users (id, name, avatar_url) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET name = $2, avatar_url = $3",
        user_info.id,
        user_info.login,
        user_info.avatar_url
    ).execute(db_pool)
    .await?;

    let mut response = Response::Ok().body(format!("Hi, {}!", user_info.login));

    // 忽略错误
    let _ = response.add_cookie(&cookie);
    Ok(response)
}
