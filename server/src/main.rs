mod article;
mod comment;
mod errors;
mod middlewares;
mod models;
mod user;
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
            .configure(|cfg| route(Arc::clone(&app_state), cfg))
    })
    .bind("0.0.0.0:12345")
    .unwrap()
    .run()
    .await
    .unwrap()
}

fn route(state: Arc<AppState>, cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/article")
            // 用FromRequest代替中间件实现身份认证
            // .wrap(middlewares::auth::CheckLogin {
            //     db_pool: state.db_pool.clone(),
            //     admin: true,
            // })
            .route("/{id}", web::get().to(article::view::get_article))
            .route("/{id}", web::delete().to(article::delete::delete_article))
            .route("", web::post().to(article::new::new_article))
            .route("", web::get().to(article::view::get_article))
            .route("", web::put().to(article::edit::edit_article))
            .route(
                "/search/{keyword}",
                web::get().to(article::search::search_article),
            ),
    )
    .service(web::scope("/articles").route("", web::get().to(article::view::get_articles_preview)))
    .service(web::scope("/user").route("/login", web::post().to(user::login::github_login)))
    .service(
        web::scope("/comment")
            .route(
                "/{article_id}",
                web::get().to(comment::view::get_comments_for_article),
            )
            .route(
                "/{comment_id}",
                web::delete().to(comment::delete::delete_comment),
            )
            .route("", web::post().to(comment::new::new_comment)),
    );
}
