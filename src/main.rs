use actix_web::{App, HttpServer, middleware};
use std::{env, io};

mod constants;
mod like;
mod response;
mod tweet;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    unsafe {
        env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    }
    env_logger::init();

    HttpServer::new(|| {
        App::new()
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
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
