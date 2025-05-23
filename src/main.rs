use actix_web::{App, HttpServer, middleware};
use diesel::{
    PgConnection,
    r2d2::{self, ConnectionManager},
};
use std::{env, io};

mod constants;
mod like;
mod response;
mod schema;
mod tweet;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    unsafe {
        env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    }
    env_logger::init();
    dotenv::dotenv().ok(); // load variables from .env file
    // set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            // Set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(tweet::get_all)
            .service(tweet::create)
            .service(tweet::get_by_id)
            .service(tweet::delete)
            .service(like::get_all_likes_by_tweet_id)
            .service(like::add_like)
            .service(like::remove_like)
    })
    .bind("0.0.0.0:9091")?
    .run()
    .await
}
