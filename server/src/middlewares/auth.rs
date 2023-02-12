use crate::models::user::GithubUserInfo;
/// 参考ntex中对DefaultHeaders这个中间件的实现
use ntex::{
    http::{
        body::{Body, ResponseBody},
        HttpMessage, Method, StatusCode,
    },
    util::BoxFuture,
    web::{WebRequest, WebResponse},
    Middleware, Service,
};
use reqwest::Client;
use sqlx::{Pool, Postgres};
use std::{task::Context, task::Poll};

pub struct CheckLogin {
    /// 数据库连接池
    pub db_pool: Pool<Postgres>,
    /// 这个操作只有管理员才能执行
    pub admin: bool,
}

pub struct CheckLoginMiddleware<S> {
    db_pool: Pool<Postgres>,
    admin: bool,
    service: S,
}

impl<S> Middleware<S> for CheckLogin {
    type Service = CheckLoginMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        CheckLoginMiddleware {
            db_pool: self.db_pool.clone(),
            admin: self.admin,
            service,
        }
    }
}

impl<S, E> Service<WebRequest<E>> for CheckLoginMiddleware<S>
where
    S: Service<WebRequest<E>, Response = WebResponse>,
    E: 'static,
{
    type Response = WebResponse;
    type Error = S::Error;
    type Future<'f> = BoxFuture<'f, Result<Self::Response, Self::Error>> where S: 'f, E: 'f;

    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: WebRequest<E>) -> Self::Future<'_> {
        Box::pin(async move {
            let requese_method = req.method().to_owned();

            // GET 请不做身份认证
            if requese_method != Method::GET {
                let db_pool = &self.db_pool;

                // Cookies 中的access token
                let cookie = req.cookie("ACCESS_TOKEN");

                // Response
                let mut res = self.service.call(req).await?;

                let access_token = match cookie {
                    Some(c) => c,
                    None => {
                        // cookies没有
                        res.response_mut().head_mut().status = StatusCode::UNAUTHORIZED;
                        res = res.map_body(|_, _| {
                            ResponseBody::from(Body::from_slice("你还没有登录".as_bytes()))
                        });
                        return Ok(res);
                    }
                };

                // HTTP Client
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
                            // status code 401 未授权
                            res.response_mut().head_mut().status = StatusCode::UNAUTHORIZED;

                            res = res.map_body(|_, _| {
                                ResponseBody::from(Body::from_slice(
                                    "获取不到 Github 用户信息，请登录".as_bytes(),
                                ))
                            });
                            return Ok(res);
                        }
                    },
                    Err(_) => {
                        // 请求错误
                        res.response_mut().head_mut().status = StatusCode::INTERNAL_SERVER_ERROR;
                        res = res.map_body(|_, _| {
                            ResponseBody::from(Body::from_slice(
                                "无法获取用户信息，请联系管理员".as_bytes(),
                            ))
                        });
                        return Ok(res);
                    }
                };

                if sqlx::query!("SELECT id FROM users WHERE id = $1", user_id)
                    .fetch_optional(db_pool)
                    .await
                    .unwrap()
                    .is_some()
                {
                    // 需要管理员权限
                    if self.admin {
                        // 管理员的 Github ID
                        if user_id == 23467070 {
                            Ok(res)
                        } else {
                            // 用户不是管理员
                            res.response_mut().head_mut().status = StatusCode::UNAUTHORIZED;
                            res = res.map_body(|_, _| {
                                ResponseBody::from(Body::from_slice("你不是网站管理员".as_bytes()))
                            });
                            Ok(res)
                        }
                    } else {
                        Ok(res)
                    }
                } else {
                    // 用户没有在本站使用Github登录过
                    res.response_mut().head_mut().status = StatusCode::UNAUTHORIZED;
                    res = res.map_body(|_, _| {
                        ResponseBody::from(Body::from_slice(
                            "你还没有在本站使用Github登录过，请登录".as_bytes(),
                        ))
                    });
                    Ok(res)
                }
            } else {
                let res = self.service.call(req).await?;
                Ok(res)
            }
        })
    }
}
