mod article;
mod errors;
mod models;
use crate::errors::CustomError;
use article::{edit, new, view};
use ntex::web::{self, middleware, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{env, sync::Arc};

pub struct AppState {
    // 数据库连接池
    pub db_pool: Pool<Postgres>,
}

#[ntex::main] // 定义异步main函数
async fn main() {
    dotenvy::dotenv().ok();

    env::set_var("RUST_LOG", "ntex=info");
    env_logger::init();

    // 从.env中获取数据库链接的环境变量，生产环境不会使用.env文件，而是在服务器上设置环境变量
    let db_url = env::var("DATABASE_URL").expect("Please set `DATABASE_URL`");

    // Arc 多重所有权；Mutex 内部可变性的互斥锁
    let app_state = Arc::new(AppState {
        db_pool: PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
            .unwrap(),
    });

    HttpServer::new(move || {
        App::new()
            .state(Arc::clone(&app_state))
            .wrap(middleware::Logger::default())
            .service(index)
            .service(error)
            .service(view::get_all_articles)
            .service(new::new_article)
            .service(edit::edit_article)
    })
    .bind("0.0.0.0:12345")
    .unwrap()
    .run()
    .await
    .unwrap()
}

#[web::get("/")]
async fn index() -> String {
    "Hello, world".into()
}

#[web::get("/error")]
async fn error() -> Result<String, CustomError> {
    Err(CustomError::NotFound("Not found".into()))
}
